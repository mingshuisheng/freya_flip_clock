use skia_safe::Canvas;

pub trait CanvasUtils {
  // fn with_clip_rect(&self,
  //                   rect: impl AsRef<Rect>, f: impl FnOnce(&Canvas));

  fn with_restore(&self, f: impl FnOnce(&Canvas));
}

impl CanvasUtils for Canvas {
  // fn with_clip_rect(&self, rect: impl AsRef<Rect>, f: impl FnOnce(&Canvas)) {
  //   self.save();
  //   self.clip_rect(rect, None, true);
  //   f(self);
  //   self.restore();
  // }
  fn with_restore(&self, f: impl FnOnce(&Canvas)) {
    self.save();
    f(self);
    self.restore();
  }
}