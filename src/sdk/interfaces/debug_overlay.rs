//VDebugOverlay003

use crate::{
    cfn, hex_to_rgb,
    math::{angles::Angles, vector::Vector3},
    vmt_call,
};

use super::WithVmt;

pub type DebugOverlay = WithVmt<VMTDebugOverlay>;
#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTDebugOverlay {
    _pad: [usize; 1],
    pub add_box_overlay: cfn!(
        (),
        &DebugOverlay,
        &Vector3,
        &Vector3,
        &Vector3,
        &Angles,
        u8,
        u8,
        u8,
        u8,
        f32
    ),
	//virtual void AddTriangleOverlay(const Vector& p1, const Vector& p2, const Vector& p3, int r, int g, int b, int a, bool noDepthTest, float duration) = 0;
    pub add_triangle_overlay: cfn!(
        (),
        &DebugOverlay,
        &Vector3,
        &Vector3,
        &Vector3,
        u8,
        u8,
        u8,
        u8,
        bool,
        f32
    ),
    //virtual void AddTriangleOverlay(const Vector& p1, const Vector& p2, const Vector& p3, int r, int g, int b, int a, bool noDepthTest, float duration) = 0;
    _pad1: [usize; 14],
    pub add_line_overlay: cfn!(
        (),
        &DebugOverlay,
        &Vector3,
        &Vector3,
        u8,
        u8,
        u8,
        u8,
        bool,
        f32
    ),
    //virtual void AddLineOverlay(const Vector& origin, const Vector& dest, int r, int g, int b,bool noDepthTest, float duration) = 0;
    //virtual void AddTextOverlay(const Vector& origin, float duration, PRINTF_FORMAT_STRING const char *format, ...) = 0;
    //virtual void AddTextOverlay(const Vector& origin, int line_offset, float duration, PRINTF_FORMAT_STRING const char *format, ...) = 0;
    //virtual void AddScreenTextOverlay(float flXPos, float flYPos,float flDuration, int r, int g, int b, int a, const char *text) = 0;
    //virtual void AddSweptBoxOverlay(const Vector& start, const Vector& end, const Vector& mins, const Vector& max, const QAngle & angles, int r, int g, int b, int a, float flDuration) = 0;
    //virtual void AddGridOverlay(const Vector& origin) = 0;
    //virtual int ScreenPosition(const Vector& point, Vector& screen) = 0;
    //virtual int ScreenPosition(float flXPos, float flYPos, Vector& screen) = 0;

    //virtual OverlayText_t *GetFirst( void ) = 0;
    //virtual OverlayText_t *GetNext( OverlayText_t *current ) = 0;
    //virtual void ClearDeadOverlays( void ) = 0;
    //virtual void ClearAllOverlays() = 0;

    //virtual void AddTextOverlayRGB(const Vector& origin, int line_offset, float duration, float r, float g, float b, float alpha, PRINTF_FORMAT_STRING const char *format, ...) = 0;
    //virtual void AddTextOverlayRGB(const Vector& origin, int line_offset, float duration, int r, int g, int b, int a, PRINTF_FORMAT_STRING const char *format, ...) = 0;

    //virtual void AddLineOverlayAlpha(const Vector& origin, const Vector& dest, int r, int g, int b, int a, bool noDepthTest, float duration) = 0;
    //virtual void AddBoxOverlay2( const Vector& origin, const Vector& mins, const Vector& max, QAngle const& orientation, const Color& faceColor, const Color& edgeColor, float duration ) = 0;
}
impl DebugOverlay {
    pub fn rect(&self, pos: &Vector3, size: f32, color: usize, alpha: u8, duration: f32) {
        let (r, g, b) = hex_to_rgb!(color);
        let size = size / 2.0;
        vmt_call!(
            self,
            add_box_overlay,
            pos,
            &(Vector3::zeroed() - size),
            &(Vector3::zeroed() + size),
            &Angles::new(0.0, 0.0, 0.0),
            r,
            g,
            b,
            alpha,
            duration
        );
    }
    pub fn triangle(&self, pos: &Vector3, size: f32, color: usize, alpha: u8, duration: f32) {
        let (r, g, b) = hex_to_rgb!(color);
        vmt_call!(
            self,
            add_triangle_overlay,
            &(Vector3::zeroed() + *pos + size),
            &(Vector3::zeroed() + *pos - size),
            &pos,
            r,
            g,
            b,
            alpha,
            true,
            duration
        );
    }
    pub fn line(&self, start: &Vector3, end: &Vector3, color: usize, alpha: u8, duration: f32) {
        let (r, g, b) = hex_to_rgb!(color);
        vmt_call!(
            self,
            add_line_overlay,
            start,
            end,
            r,
            g,
            b,
            alpha,
            true,
            duration
        );
    }
}
