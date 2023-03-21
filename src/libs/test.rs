use crate::libs::*;


#[test]
fn test_1080p() {
  let width: u32 = 1920;
  let height: u32 = 1080;
  let test: Ressolution = Ressolution::new(width, height);

  let target1: (u32, u32) = (16, 9);
  let target2: u32 = width * height;

  assert_eq!(test.get_ar(), target1);
  assert_eq!(test.get_p_count(), target2);
}
#[test]
fn test_720p() {
  let width: u32 = 1280;
  let height: u32 = 720;
  let test: Ressolution = Ressolution::new(width, height);
  
  let target1: (u32, u32) = (16, 9);
  let target2: u32 = width * height;


  assert_eq!(test.get_ar(), target1);
  assert_eq!(test.get_p_count(), target2);
}