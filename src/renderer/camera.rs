extern crate glium;

use cgmath::{Point3, Vector3, Matrix4};

// 60fps, more or less 60 units per second
const CAMERA_SPEED: f32 = 60.0;

#[derive(Copy, Clone,Debug)]
pub struct Camera{
    view_eye: Point3<f32>,
    view_center: Point3<f32>,
    view_up: Vector3<f32>,
    target_eye: Point3<f32>,
}

impl Camera{
    pub fn new(eye_pos: Point3<f32>, view_center: Point3<f32>) -> Camera {
        Camera{
            view_eye: eye_pos,
            view_center:  view_center,
            view_up:  Vector3::new(0.0, 1.0, 0.0),
            target_eye: eye_pos,
        }
    }

//    #[inline]
//    pub fn get_eye(self) -> Point3<f32>{
//        self.view_eye
//    }

    #[inline]
    pub fn change_elevation(&mut self, target:f32)
    {
    //    print!("order: {}  -> ", target);
        let to;
        if self.is_still(){
            to = Point3::new(self.view_eye.x, 
                             self.view_eye.y + target, 
                             self.view_eye.z);
        }
        else{
            to = Point3::new(self.target_eye.x, 
                             self.target_eye.y + target, 
                             self.target_eye.z);
        }
        self.move_to(to);
    }

    #[inline]
    pub fn move_to(&mut self, target: Point3<f32>)
    {
        self.target_eye = target;
     //   print!("goto: {:?}\n", self.target_eye);
    }

    #[inline]
    pub fn is_still(self) -> bool  {
        use cgmath::ApproxEq;
        self.target_eye.approx_eq_eps(&self.view_eye, &0.5) 
    }

    #[inline]
    pub fn update(&mut self, delta: f32)
    {
        use cgmath::EuclideanSpace;
        use cgmath::InnerSpace;

        if self.is_still() { 
            return; 
        }

        let step = CAMERA_SPEED * delta;
        let vector = self.target_eye.to_vec() - self.view_eye.to_vec();

   //     print!("v {:?} \n", vector);
        if vector.magnitude() < step{
            self.view_eye = self.target_eye;
            return;
        }

        self.view_eye = self.view_eye + vector.normalize()*step;
     //   print!("at! {:?} -> {:?}\n", self.view_eye, self.target_eye);
    }

    pub fn get_eye(&self) -> Point3<f32>{
        self.view_eye
    }
}

impl Into<Matrix4<f32>> for Camera {
	#[inline]
	fn into(self) -> Matrix4<f32>{
		Matrix4::look_at(self.view_eye, self.view_center, self.view_up)
	}
}

#[cfg(test)]
mod tests {

    use super::Camera;
    use cgmath::{Point3, Matrix4};


    #[test]
    fn create() {
	     Camera::new(Point3::new(0.0, 75.0, -110.0),Point3::new(0.0, 0.0, -0.0));
    }
    #[test]
    fn into_matrix() {
	     let cam = Camera::new(Point3::new(0.0, 75.0, -110.0),Point3::new(0.0, 0.0, -0.0));
        let _ : Matrix4<f32> = cam.into();
    }
    #[test]
    fn target() {
	     let mut cam = Camera::new(Point3::new(0.0, 75.0, -110.0),Point3::new(0.0, 0.0, -0.0));
         cam.move_to(Point3::new(0.0, 75.0, -105.0));
        
            print!("{:?}\n", cam);
            cam.update(1.1);
            print!("{:?}\n", cam);
            cam.update(1.1);
            print!("{:?}\n", cam);
            cam.update(1.1);
            print!("{:?}\n", cam);
            cam.update(1.1);
            print!("{:?}\n", cam);
            cam.update(1.1);
            print!("{:?}\n", cam);

        assert!(cam.is_still());
    }
    #[test]
    fn target2() {
	     let mut cam = Camera::new(Point3::new(0.0, 75.0, -110.0),Point3::new(0.0, 0.0, -0.0));
         cam.change_elevation(5.0);
          //  print!("{:?}\n", cam);
            cam.update(1.1);
          //  print!("{:?}\n", cam);
            cam.update(1.1);
          //  print!("{:?}\n", cam);
            cam.update(1.1);
          //  print!("{:?}\n", cam);
            cam.update(1.1);
          //  print!("{:?}\n", cam);
            cam.update(1.1);
          //  print!("{:?}\n", cam);
        assert!(cam.is_still());
    }
}
