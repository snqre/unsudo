








pub struct Element(&'static str); 

impl Element {
    pub fn new(identifier: &'static str) -> Self {
        Self(identifier)
    }

    pub fn class(&self) -> Option<Vec<String>> {
        let ret: Vec<String> = self
            .dom_element()?
            .class_name()
            .split_whitespace()
            .map(|s| s.to_owned())
            .collect();
        Some(ret)
    }

    pub fn center_x(&self) -> Option<f64> {
        let ret: f64 = self.center()?.0;
        Some(ret)
    }

    pub fn center_y(&self) -> Option<f64> {
        let ret: f64 = self.center()?.1;
        Some(ret)
    }

    pub fn center(&self) -> Option<(f64, f64)> {
        let (x, y) = self.coordinate()?;
        let (w, h) = self.size()?;
        Some((x + w / 2.0, y + h / 2.0))
    }

    pub fn w(&self) -> Option<f64> {
        let ret: f64 = self.size()?.0;
        Some(ret)
    }

    pub fn h(&self) -> Option<f64> {
        let ret: f64 = self.size()?.1;
        Some(ret)
    }

    pub fn size(&self) -> Option<(f64, f64)> {
        let dom_rect: web_sys::DomRect = self.dom_rect()?;
        let dom_rect_w: f64 = dom_rect.width();
        let dom_rect_h: f64 = dom_rect.height();
        Some((dom_rect_w, dom_rect_h))
    }

    pub fn x(&self) -> Option<f64> {
        let ret: f64 = self.coordinate()?.0;
        Some(ret)
    }
    
    pub fn y(&self) -> Option<f64> {
        let ret: f64 = self.coordinate()?.1;
        Some(ret)
    }

    pub fn coordinate(&self) -> Option<(f64, f64)> {
        let dom_rect: web_sys::DomRect = self.dom_rect()?;
        let dom_rect_x: f64 = dom_rect.x();
        let dom_rect_y: f64 = dom_rect.y();
        Some((dom_rect_x, dom_rect_y))
    }

    fn dom_rect(&self) -> Option<web_sys::DomRect> {
        let ret: web_sys::DomRect = self.dom_element()?.get_bounding_client_rect();
        Some(ret)
    }

    fn dom_element(&self) -> Option<web_sys::Element> {
        web_sys::window()?.document()?.get_element_by_id(self.0)
    }
}