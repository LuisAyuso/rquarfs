// a stage has inputs and outputs.
// once all inputs are satisfied, the stage itself
// can be executed

use std::collections::BTreeSet;
use std::collections::VecDeque;
use std::marker::PhantomData;

pub struct StageInstance<CTX, T, F>
    where F: Fn(&mut CTX, &[T])
{
    name: &'static str,
    inputs: Vec<T>,
    outputs: Vec<T>,
    func: F,
    ghost: PhantomData<CTX>,
}

impl<CTX, T, F> StageInstance<CTX, T, F>
    where T: Clone,
          F: Fn(&mut CTX, &[T])
{
    pub fn new(name: &'static str, inputs: &[T], outputs: &[T], f: F) -> StageInstance<CTX, T, F> {
        StageInstance {
            name: name,
            inputs: inputs.to_vec(),
            outputs: outputs.to_vec(),
            func: f,
            ghost: PhantomData,
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub trait Stage<CTX, T> {
    fn get_inputs(&self) -> &[T];
    fn get_outputs(&self) -> &[T];
    fn get_name(&self) -> &'static str;
    fn execute(&self, ctx: &mut CTX);
}

impl<CTX, T, F> Stage<CTX, T> for StageInstance<CTX, T, F>
    where F: Fn(&mut CTX, &[T])
{
    fn get_inputs(&self) -> &[T] {
        &self.inputs
    }
    // TODO: it could be more robust if outputs happen as a
    // side effect
    fn get_outputs(&self) -> &[T] {
        &self.outputs
    }

    fn get_name(&self) -> &'static str {
        self.name
    }

    fn execute(&self, ctx: &mut CTX) {
        (self.func)(ctx, self.inputs.as_slice());
    }
}

type BStage<CTX, T> = Box<Stage<CTX, T>>;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub struct Pipeline<CTX, T>
    where T: Ord
{
    queue: Vec<BStage<CTX, T>>,
}

impl<CTX, T> Pipeline<CTX, T>
    where T: Ord
{
    pub fn new(_: &CTX) -> Pipeline<CTX, T> {
        Pipeline { queue: Vec::new() }
    }

    pub fn queue(&mut self, stage: BStage<CTX, T>) {
        self.queue.push(stage);
    }

    fn get_stages(&self) -> &Vec<BStage<CTX, T>> {
        &self.queue
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

struct Runner<'a, CTX: 'a, T: Ord + 'a> {
    available: BTreeSet<T>,
    // pipe: &'a Pipeline<T>,
    ready: VecDeque<&'a BStage<CTX, T>>,
    wait: Vec<&'a BStage<CTX, T>>,
}
impl<'a, CTX, T> Runner<'a, CTX, T>
    where T: Ord + Copy + 'a
{
    fn new(pipe: &'a Pipeline<CTX, T>) -> Runner<'a, CTX, T> {
        Runner {
            available: BTreeSet::new(),
            ready: pipe.get_stages()
                .iter()
                .filter(|x| x.get_inputs().is_empty())
                .collect::<VecDeque<&BStage<CTX, T>>>(),
            wait: pipe.get_stages()
                .iter()
                .filter(|x| !x.get_inputs().is_empty())
                .collect::<Vec<&BStage<CTX, T>>>(),
        }
    }

    fn pop(&mut self) -> Option<&'a BStage<CTX, T>> {
        self.ready.pop_front()
    }

    /// commits the end of an stage, notifies
    /// tasks that can be executed afterwards
    fn commit(&mut self, outputs: &[T]) {
        // make outputs available
        for o in outputs.iter() {
            self.available.insert(*o);
        }
        // make ready all stages whit satisified requisites
        let new_ready = self.wait
            .iter()
            .filter(|s| {
                s.get_inputs()
                    .iter()
                    .map(|i| self.available.contains(i))
                    .fold(true, |acu, c| acu && c)
            })
            .map(|x| *x)
            .collect::<Vec<&BStage<CTX, T>>>();
        self.ready.extend(new_ready.as_slice());
        self.wait = self.wait
            .iter()
            .filter(|s| {
                !s.get_inputs()
                    .iter()
                    .map(|i| self.available.contains(i))
                    .fold(true, |acu, c| acu && c)
            })
            .map(|x| *x)
            .collect::<Vec<&BStage<CTX, T>>>();
    }

    fn is_done(&self) -> bool {
        self.wait.is_empty()
    }
}

// =============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stage() {
        let a = "a";
        let b = "b";
        let c = "c";

        let inputs = vec![a, b, c];

        let _ = StageInstance::new("stage", inputs.as_slice(), &[], |ctx: &mut i32, inputs| {
            println!("{} {:?}", ctx, inputs);
        });
    }

    #[test]
    fn pipeline() {

        println!("hello!");
        let mut ctx: u32 = 0;
        let mut pipe = Pipeline::new(&ctx);

        pipe.queue(Box::new(StageInstance::new("stage",
                                               &[],
                                               &["a"],
                                               |ctx, inputs| println!("{} {:?}", ctx, inputs))));
        pipe.queue(Box::new(StageInstance::new("stage",
                                               &["a"],
                                               &["b"],
                                               |ctx, inputs| println!("{} {:?}", ctx, inputs))));
        pipe.queue(Box::new(StageInstance::new("stage",
                                               &["a"],
                                               &["c"],
                                               |ctx, inputs| println!("{} {:?}", ctx, inputs))));
        pipe.queue(Box::new(StageInstance::new("stage",
                                               &["c", "b"],
                                               &["d"],
                                               |ctx, inputs| println!("{} {:?}", ctx, inputs))));
        pipe.queue(Box::new(StageInstance::new("stage",
                                               &["a", "d"],
                                               &[],
                                               |ctx, inputs| println!("{} {:?}", ctx, inputs))));

        let mut runner = Runner::new(&pipe);
        while let Some(x) = runner.pop() {
            x.execute(&mut ctx);
            runner.commit(x.get_outputs());
        }
        assert!(runner.is_done());
    }


    #[test]
    fn mutable_state() {

        println!("hello!");
        let mut state = 0;
        let mut pipe = Pipeline::new(&state);

        pipe.queue(Box::new(StageInstance::new("stage", &[], &[1], |ctx, _| *ctx = 1)));
        pipe.queue(Box::new(StageInstance::new("stage", &[1], &[2], |ctx, _| *ctx = *ctx + 1)));
        pipe.queue(Box::new(StageInstance::new("stage", &[2], &[3], |ctx, _| *ctx = *ctx + 1)));

        let mut runner = Runner::new(&pipe);
        while let Some(x) = runner.pop() {
            x.execute(&mut state);
            runner.commit(x.get_outputs());
        }
        assert!(runner.is_done());
        assert!(state == 3);
    }
}
