#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - LTDC identification register"]
    pub ltdc_idr: LTDC_IDR,
    #[doc = "0x04 - LDTC layer count register"]
    pub ltdc_lcr: LTDC_LCR,
    #[doc = "0x08 - This register defines the number of horizontal synchronization pixels minus 1 and the number of vertical synchronization lines minus 1. Refer to Figure274 and Section36.4: LTDC programmable parameters for an example of configuration."]
    pub ltdc_sscr: LTDC_SSCR,
    #[doc = "0x0c - This register defines the accumulated number of horizontal synchronization and back porch pixels minus 1 (HSYNCwidth+HBP-1) and the accumulated number of vertical synchronization and back porch lines minus 1 (VSYNCheight+VBP-1). Refer to Figure274 and Section36.4: LTDC programmable parameters for an example of configuration."]
    pub ltdc_bpcr: LTDC_BPCR,
    #[doc = "0x10 - This register defines the accumulated number of horizontal synchronization, back porch and active pixels minus 1 (HSYNC width+HBP+activewidth-1) and the accumulated number of vertical synchronization, back porch lines and active lines minus 1 (VSYNCheight+BVBP+activeheight-1). Refer to Figure274 and Section36.4: LTDC programmable parameters for an example of configuration."]
    pub ltdc_awcr: LTDC_AWCR,
    #[doc = "0x14 - This register defines the accumulated number of horizontal synchronization, back porch, active and front porch pixels minus 1 (HSYNCwidth+HBP+activewidth+HFP-1) and the accumulated number of vertical synchronization, back porch lines, active and front lines minus 1 (VSYNCheight+BVBP+activeheight+VFP-1). Refer to Figure274 and Section36.4: LTDC programmable parameters for an example of configuration."]
    pub ltdc_twcr: LTDC_TWCR,
    #[doc = "0x18 - This register defines the global configuration of the LCD-TFT controller."]
    pub ltdc_gcr: LTDC_GCR,
    #[doc = "0x1c - LTDC global configuration 1 register"]
    pub ltdc_gc1r: LTDC_GC1R,
    #[doc = "0x20 - LTDC global configuration 2 register"]
    pub ltdc_gc2r: LTDC_GC2R,
    #[doc = "0x24 - This register allows to reload either immediately or during the vertical blanking period, the shadow registers values to the active registers. The shadow registers are all Layer1 and Layer2 registers except the LTDC_L1CLUTWR and the LTDC_L2CLUTWR."]
    pub ltdc_srcr: LTDC_SRCR,
    _reserved10: [u8; 4usize],
    #[doc = "0x2c - This register defines the background color (RGB888)."]
    pub ltdc_bccr: LTDC_BCCR,
    _reserved11: [u8; 4usize],
    #[doc = "0x34 - This register determines which status flags generate an interrupt request by setting the corresponding bit to 1."]
    pub ltdc_ier: LTDC_IER,
    #[doc = "0x38 - This register returns the interrupt status flag."]
    pub ltdc_isr: LTDC_ISR,
    #[doc = "0x3c - LTDC Interrupt Clear Register"]
    pub ltdc_icr: LTDC_ICR,
    #[doc = "0x40 - This register defines the position of the line interrupt. The line value to be programmed depends on the timings parameters. Refer to Figure274."]
    pub ltdc_lipcr: LTDC_LIPCR,
    #[doc = "0x44 - LTDC current position status register"]
    pub ltdc_cpsr: LTDC_CPSR,
    #[doc = "0x48 - This register returns the status of the current display phase which is controlled by the HSYNC, VSYNC, and horizontal/vertical DE signals. Example: if the current display phase is the vertical synchronization, the VSYNCS bit is set (active high). If the current display phase is the horizontal synchronization, the HSYNCS bit is active high."]
    pub ltdc_cdsr: LTDC_CDSR,
    _reserved17: [u8; 56usize],
    #[doc = "0x84 - LTDC layer 1 control register"]
    pub ltdc_l1cr: LTDC_L1CR,
    #[doc = "0x88 - This register defines the horizontal position (first and last pixel) of the layer 1 or 2 window. The first visible pixel of a line is the programmed value of AHBP\\[11:0\\]
bits + 1 in the LTDC_BPCR register. The last visible pixel of a line is the programmed value of AAW\\[11:0\\]
bits in the LTDC_AWCR register."]
    pub ltdc_l1whpcr: LTDC_L1WHPCR,
    #[doc = "0x8c - This register defines the vertical position (first and last line) of the layer1 or 2 window. The first visible line of a frame is the programmed value of AVBP\\[11:0\\]
bits + 1 in the register LTDC_BPCR register. The last visible line of a frame is the programmed value of AAH\\[11:0\\]
bits in the LTDC_AWCR register."]
    pub ltdc_l1wvpcr: LTDC_L1WVPCR,
    #[doc = "0x90 - This register defines the color key value (RGB), that is used by the color keying."]
    pub ltdc_l1ckcr: LTDC_L1CKCR,
    #[doc = "0x94 - This register defines the pixel format that is used for the stored data in the frame buffer of a layer. The pixel data is read from the frame buffer and then transformed to the internal format 8888 (ARGB)."]
    pub ltdc_l1pfcr: LTDC_L1PFCR,
    #[doc = "0x98 - This register defines the constant alpha value (divided by 255 by hardware), that is used in the alpha blending. Refer to LTDC_LxBFCR register."]
    pub ltdc_l1cacr: LTDC_L1CACR,
    #[doc = "0x9c - This register defines the default color of a layer in the format ARGB. The default color is used outside the defined layer window or when a layer is disabled. The reset value of 0x00000000 defines a transparent black color."]
    pub ltdc_l1dccr: LTDC_L1DCCR,
    #[doc = "0xa0 - This register defines the blending factors F1 and F2. The general blending formula is: BC = BF1 x C + BF2 x Cs BC = blended color BF1 = blend factor 1 C = current layer color BF2 = blend factor 2 Cs = subjacent layers blended color"]
    pub ltdc_l1bfcr: LTDC_L1BFCR,
    _reserved25: [u8; 8usize],
    #[doc = "0xac - This register defines the color frame buffer start address which has to point to the address where the pixel data of the top left pixel of a layer is stored in the frame buffer."]
    pub ltdc_l1cfbar: LTDC_L1CFBAR,
    #[doc = "0xb0 - This register defines the color frame buffer line length and pitch."]
    pub ltdc_l1cfblr: LTDC_L1CFBLR,
    #[doc = "0xb4 - This register defines the number of lines in the color frame buffer."]
    pub ltdc_l1cfblnr: LTDC_L1CFBLNR,
    _reserved28: [u8; 12usize],
    #[doc = "0xc4 - This register defines the CLUT address and the RGB value."]
    pub ltdc_l1clutwr: LTDC_L1CLUTWR,
    _reserved29: [u8; 60usize],
    #[doc = "0x104 - LTDC layer 2 control register"]
    pub ltdc_l2cr: LTDC_L2CR,
    #[doc = "0x108 - This register defines the horizontal position (first and last pixel) of the layer 1 or 2 window. The first visible pixel of a line is the programmed value of AHBP\\[11:0\\]
bits + 1 in the LTDC_BPCR register. The last visible pixel of a line is the programmed value of AAW\\[11:0\\]
bits in the LTDC_AWCR register."]
    pub ltdc_l2whpcr: LTDC_L2WHPCR,
    #[doc = "0x10c - This register defines the vertical position (first and last line) of the layer1 or 2 window. The first visible line of a frame is the programmed value of AVBP\\[11:0\\]
bits + 1 in the register LTDC_BPCR register. The last visible line of a frame is the programmed value of AAH\\[11:0\\]
bits in the LTDC_AWCR register."]
    pub ltdc_l2wvpcr: LTDC_L2WVPCR,
    #[doc = "0x110 - This register defines the color key value (RGB), that is used by the color keying."]
    pub ltdc_l2ckcr: LTDC_L2CKCR,
    #[doc = "0x114 - This register defines the pixel format that is used for the stored data in the frame buffer of a layer. The pixel data is read from the frame buffer and then transformed to the internal format 8888 (ARGB)."]
    pub ltdc_l2pfcr: LTDC_L2PFCR,
    #[doc = "0x118 - This register defines the constant alpha value (divided by 255 by hardware), that is used in the alpha blending. Refer to LTDC_LxBFCR register."]
    pub ltdc_l2cacr: LTDC_L2CACR,
    #[doc = "0x11c - This register defines the default color of a layer in the format ARGB. The default color is used outside the defined layer window or when a layer is disabled. The reset value of 0x00000000 defines a transparent black color."]
    pub ltdc_l2dccr: LTDC_L2DCCR,
    #[doc = "0x120 - This register defines the blending factors F1 and F2. The general blending formula is: BC = BF1 x C + BF2 x Cs BC = blended color BF1 = blend factor 1 C = current layer color BF2 = blend factor 2 Cs = subjacent layers blended color"]
    pub ltdc_l2bfcr: LTDC_L2BFCR,
    _reserved37: [u8; 8usize],
    #[doc = "0x12c - This register defines the color frame buffer start address which has to point to the address where the pixel data of the top left pixel of a layer is stored in the frame buffer."]
    pub ltdc_l2cfbar: LTDC_L2CFBAR,
    #[doc = "0x130 - This register defines the color frame buffer line length and pitch."]
    pub ltdc_l2cfblr: LTDC_L2CFBLR,
    #[doc = "0x134 - This register defines the number of lines in the color frame buffer."]
    pub ltdc_l2cfblnr: LTDC_L2CFBLNR,
    _reserved40: [u8; 12usize],
    #[doc = "0x144 - This register defines the CLUT address and the RGB value."]
    pub ltdc_l2clutwr: LTDC_L2CLUTWR,
}
#[doc = "LTDC identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_idr](ltdc_idr) module"]
pub type LTDC_IDR = crate::Reg<u32, _LTDC_IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTDC_IDR;
#[doc = "`read()` method returns [ltdc_idr::R](ltdc_idr::R) reader structure"]
impl crate::Readable for LTDC_IDR {}
#[doc = "LTDC identification register"]
pub mod ltdc_idr;
#[doc = "LDTC layer count register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_lcr](ltdc_lcr) module"]
pub type LTDC_LCR = crate::Reg<u32, _LTDC_LCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTDC_LCR;
#[doc = "`read()` method returns [ltdc_lcr::R](ltdc_lcr::R) reader structure"]
impl crate::Readable for LTDC_LCR {}
#[doc = "LDTC layer count register"]
pub mod ltdc_lcr;
#[doc = "This register defines the number of horizontal synchronization pixels minus 1 and the number of vertical synchronization lines minus 1. Refer to Figure274 and Section36.4: LTDC programmable parameters for an example of configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_sscr](ltdc_sscr) module"]
pub type LTDC_SSCR = crate::Reg<u32, _LTDC_SSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTDC_SSCR;
#[doc = "`read()` method returns [ltdc_sscr::R](ltdc_sscr::R) reader structure"]
impl crate::Readable for LTDC_SSCR {}
#[doc = "`write(|w| ..)` method takes [ltdc_sscr::W](ltdc_sscr::W) writer structure"]
impl crate::Writable for LTDC_SSCR {}
#[doc = "This register defines the number of horizontal synchronization pixels minus 1 and the number of vertical synchronization lines minus 1. Refer to Figure274 and Section36.4: LTDC programmable parameters for an example of configuration."]
pub mod ltdc_sscr;
#[doc = "This register defines the accumulated number of horizontal synchronization and back porch pixels minus 1 (HSYNCwidth+HBP-1) and the accumulated number of vertical synchronization and back porch lines minus 1 (VSYNCheight+VBP-1). Refer to Figure274 and Section36.4: LTDC programmable parameters for an example of configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_bpcr](ltdc_bpcr) module"]
pub type LTDC_BPCR = crate::Reg<u32, _LTDC_BPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTDC_BPCR;
#[doc = "`read()` method returns [ltdc_bpcr::R](ltdc_bpcr::R) reader structure"]
impl crate::Readable for LTDC_BPCR {}
#[doc = "`write(|w| ..)` method takes [ltdc_bpcr::W](ltdc_bpcr::W) writer structure"]
impl crate::Writable for LTDC_BPCR {}
#[doc = "This register defines the accumulated number of horizontal synchronization and back porch pixels minus 1 (HSYNCwidth+HBP-1) and the accumulated number of vertical synchronization and back porch lines minus 1 (VSYNCheight+VBP-1). Refer to Figure274 and Section36.4: LTDC programmable parameters for an example of configuration."]
pub mod ltdc_bpcr;
#[doc = "This register defines the accumulated number of horizontal synchronization, back porch and active pixels minus 1 (HSYNC width+HBP+activewidth-1) and the accumulated number of vertical synchronization, back porch lines and active lines minus 1 (VSYNCheight+BVBP+activeheight-1). Refer to Figure274 and Section36.4: LTDC programmable parameters for an example of configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_awcr](ltdc_awcr) module"]
pub type LTDC_AWCR = crate::Reg<u32, _LTDC_AWCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTDC_AWCR;
#[doc = "`read()` method returns [ltdc_awcr::R](ltdc_awcr::R) reader structure"]
impl crate::Readable for LTDC_AWCR {}
#[doc = "`write(|w| ..)` method takes [ltdc_awcr::W](ltdc_awcr::W) writer structure"]
impl crate::Writable for LTDC_AWCR {}
#[doc = "This register defines the accumulated number of horizontal synchronization, back porch and active pixels minus 1 (HSYNC width+HBP+activewidth-1) and the accumulated number of vertical synchronization, back porch lines and active lines minus 1 (VSYNCheight+BVBP+activeheight-1). Refer to Figure274 and Section36.4: LTDC programmable parameters for an example of configuration."]
pub mod ltdc_awcr;
#[doc = "This register defines the accumulated number of horizontal synchronization, back porch, active and front porch pixels minus 1 (HSYNCwidth+HBP+activewidth+HFP-1) and the accumulated number of vertical synchronization, back porch lines, active and front lines minus 1 (VSYNCheight+BVBP+activeheight+VFP-1). Refer to Figure274 and Section36.4: LTDC programmable parameters for an example of configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_twcr](ltdc_twcr) module"]
pub type LTDC_TWCR = crate::Reg<u32, _LTDC_TWCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTDC_TWCR;
#[doc = "`read()` method returns [ltdc_twcr::R](ltdc_twcr::R) reader structure"]
impl crate::Readable for LTDC_TWCR {}
#[doc = "`write(|w| ..)` method takes [ltdc_twcr::W](ltdc_twcr::W) writer structure"]
impl crate::Writable for LTDC_TWCR {}
#[doc = "This register defines the accumulated number of horizontal synchronization, back porch, active and front porch pixels minus 1 (HSYNCwidth+HBP+activewidth+HFP-1) and the accumulated number of vertical synchronization, back porch lines, active and front lines minus 1 (VSYNCheight+BVBP+activeheight+VFP-1). Refer to Figure274 and Section36.4: LTDC programmable parameters for an example of configuration."]
pub mod ltdc_twcr;
#[doc = "This register defines the global configuration of the LCD-TFT controller.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_gcr](ltdc_gcr) module"]
pub type LTDC_GCR = crate::Reg<u32, _LTDC_GCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTDC_GCR;
#[doc = "`read()` method returns [ltdc_gcr::R](ltdc_gcr::R) reader structure"]
impl crate::Readable for LTDC_GCR {}
#[doc = "`write(|w| ..)` method takes [ltdc_gcr::W](ltdc_gcr::W) writer structure"]
impl crate::Writable for LTDC_GCR {}
#[doc = "This register defines the global configuration of the LCD-TFT controller."]
pub mod ltdc_gcr;
#[doc = "LTDC global configuration 1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_gc1r](ltdc_gc1r) module"]
pub type LTDC_GC1R = crate::Reg<u32, _LTDC_GC1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTDC_GC1R;
#[doc = "`read()` method returns [ltdc_gc1r::R](ltdc_gc1r::R) reader structure"]
impl crate::Readable for LTDC_GC1R {}
#[doc = "LTDC global configuration 1 register"]
pub mod ltdc_gc1r;
#[doc = "LTDC global configuration 2 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_gc2r](ltdc_gc2r) module"]
pub type LTDC_GC2R = crate::Reg<u32, _LTDC_GC2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTDC_GC2R;
#[doc = "`read()` method returns [ltdc_gc2r::R](ltdc_gc2r::R) reader structure"]
impl crate::Readable for LTDC_GC2R {}
#[doc = "LTDC global configuration 2 register"]
pub mod ltdc_gc2r;
#[doc = "This register allows to reload either immediately or during the vertical blanking period, the shadow registers values to the active registers. The shadow registers are all Layer1 and Layer2 registers except the LTDC_L1CLUTWR and the LTDC_L2CLUTWR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_srcr](ltdc_srcr) module"]
pub type LTDC_SRCR = crate::Reg<u32, _LTDC_SRCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTDC_SRCR;
#[doc = "`read()` method returns [ltdc_srcr::R](ltdc_srcr::R) reader structure"]
impl crate::Readable for LTDC_SRCR {}
#[doc = "`write(|w| ..)` method takes [ltdc_srcr::W](ltdc_srcr::W) writer structure"]
impl crate::Writable for LTDC_SRCR {}
#[doc = "This register allows to reload either immediately or during the vertical blanking period, the shadow registers values to the active registers. The shadow registers are all Layer1 and Layer2 registers except the LTDC_L1CLUTWR and the LTDC_L2CLUTWR."]
pub mod ltdc_srcr;
#[doc = "This register defines the background color (RGB888).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_bccr](ltdc_bccr) module"]
pub type LTDC_BCCR = crate::Reg<u32, _LTDC_BCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTDC_BCCR;
#[doc = "`read()` method returns [ltdc_bccr::R](ltdc_bccr::R) reader structure"]
impl crate::Readable for LTDC_BCCR {}
#[doc = "`write(|w| ..)` method takes [ltdc_bccr::W](ltdc_bccr::W) writer structure"]
impl crate::Writable for LTDC_BCCR {}
#[doc = "This register defines the background color (RGB888)."]
pub mod ltdc_bccr;
#[doc = "This register determines which status flags generate an interrupt request by setting the corresponding bit to 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_ier](ltdc_ier) module"]
pub type LTDC_IER = crate::Reg<u32, _LTDC_IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTDC_IER;
#[doc = "`read()` method returns [ltdc_ier::R](ltdc_ier::R) reader structure"]
impl crate::Readable for LTDC_IER {}
#[doc = "`write(|w| ..)` method takes [ltdc_ier::W](ltdc_ier::W) writer structure"]
impl crate::Writable for LTDC_IER {}
#[doc = "This register determines which status flags generate an interrupt request by setting the corresponding bit to 1."]
pub mod ltdc_ier;
#[doc = "This register returns the interrupt status flag.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_isr](ltdc_isr) module"]
pub type LTDC_ISR = crate::Reg<u32, _LTDC_ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTDC_ISR;
#[doc = "`read()` method returns [ltdc_isr::R](ltdc_isr::R) reader structure"]
impl crate::Readable for LTDC_ISR {}
#[doc = "This register returns the interrupt status flag."]
pub mod ltdc_isr;
#[doc = "LTDC Interrupt Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_icr](ltdc_icr) module"]
pub type LTDC_ICR = crate::Reg<u32, _LTDC_ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTDC_ICR;
#[doc = "`write(|w| ..)` method takes [ltdc_icr::W](ltdc_icr::W) writer structure"]
impl crate::Writable for LTDC_ICR {}
#[doc = "LTDC Interrupt Clear Register"]
pub mod ltdc_icr;
#[doc = "This register defines the position of the line interrupt. The line value to be programmed depends on the timings parameters. Refer to Figure274.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_lipcr](ltdc_lipcr) module"]
pub type LTDC_LIPCR = crate::Reg<u32, _LTDC_LIPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTDC_LIPCR;
#[doc = "`read()` method returns [ltdc_lipcr::R](ltdc_lipcr::R) reader structure"]
impl crate::Readable for LTDC_LIPCR {}
#[doc = "`write(|w| ..)` method takes [ltdc_lipcr::W](ltdc_lipcr::W) writer structure"]
impl crate::Writable for LTDC_LIPCR {}
#[doc = "This register defines the position of the line interrupt. The line value to be programmed depends on the timings parameters. Refer to Figure274."]
pub mod ltdc_lipcr;
#[doc = "LTDC current position status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_cpsr](ltdc_cpsr) module"]
pub type LTDC_CPSR = crate::Reg<u32, _LTDC_CPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTDC_CPSR;
#[doc = "`read()` method returns [ltdc_cpsr::R](ltdc_cpsr::R) reader structure"]
impl crate::Readable for LTDC_CPSR {}
#[doc = "LTDC current position status register"]
pub mod ltdc_cpsr;
#[doc = "This register returns the status of the current display phase which is controlled by the HSYNC, VSYNC, and horizontal/vertical DE signals. Example: if the current display phase is the vertical synchronization, the VSYNCS bit is set (active high). If the current display phase is the horizontal synchronization, the HSYNCS bit is active high.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_cdsr](ltdc_cdsr) module"]
pub type LTDC_CDSR = crate::Reg<u32, _LTDC_CDSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTDC_CDSR;
#[doc = "`read()` method returns [ltdc_cdsr::R](ltdc_cdsr::R) reader structure"]
impl crate::Readable for LTDC_CDSR {}
#[doc = "This register returns the status of the current display phase which is controlled by the HSYNC, VSYNC, and horizontal/vertical DE signals. Example: if the current display phase is the vertical synchronization, the VSYNCS bit is set (active high). If the current display phase is the horizontal synchronization, the HSYNCS bit is active high."]
pub mod ltdc_cdsr;
#[doc = "LTDC layer 1 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_l1cr](ltdc_l1cr) module"]
pub type LTDC_L1CR = crate::Reg<u32, _LTDC_L1CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTDC_L1CR;
#[doc = "`read()` method returns [ltdc_l1cr::R](ltdc_l1cr::R) reader structure"]
impl crate::Readable for LTDC_L1CR {}
#[doc = "`write(|w| ..)` method takes [ltdc_l1cr::W](ltdc_l1cr::W) writer structure"]
impl crate::Writable for LTDC_L1CR {}
#[doc = "LTDC layer 1 control register"]
pub mod ltdc_l1cr;
#[doc = "This register defines the horizontal position (first and last pixel) of the layer 1 or 2 window. The first visible pixel of a line is the programmed value of AHBP\\[11:0\\]
bits + 1 in the LTDC_BPCR register. The last visible pixel of a line is the programmed value of AAW\\[11:0\\]
bits in the LTDC_AWCR register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_l1whpcr](ltdc_l1whpcr) module"]
pub type LTDC_L1WHPCR = crate::Reg<u32, _LTDC_L1WHPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTDC_L1WHPCR;
#[doc = "`read()` method returns [ltdc_l1whpcr::R](ltdc_l1whpcr::R) reader structure"]
impl crate::Readable for LTDC_L1WHPCR {}
#[doc = "`write(|w| ..)` method takes [ltdc_l1whpcr::W](ltdc_l1whpcr::W) writer structure"]
impl crate::Writable for LTDC_L1WHPCR {}
#[doc = "This register defines the horizontal position (first and last pixel) of the layer 1 or 2 window. The first visible pixel of a line is the programmed value of AHBP\\[11:0\\]
bits + 1 in the LTDC_BPCR register. The last visible pixel of a line is the programmed value of AAW\\[11:0\\]
bits in the LTDC_AWCR register."]
pub mod ltdc_l1whpcr;
#[doc = "This register defines the vertical position (first and last line) of the layer1 or 2 window. The first visible line of a frame is the programmed value of AVBP\\[11:0\\]
bits + 1 in the register LTDC_BPCR register. The last visible line of a frame is the programmed value of AAH\\[11:0\\]
bits in the LTDC_AWCR register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_l1wvpcr](ltdc_l1wvpcr) module"]
pub type LTDC_L1WVPCR = crate::Reg<u32, _LTDC_L1WVPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTDC_L1WVPCR;
#[doc = "`read()` method returns [ltdc_l1wvpcr::R](ltdc_l1wvpcr::R) reader structure"]
impl crate::Readable for LTDC_L1WVPCR {}
#[doc = "`write(|w| ..)` method takes [ltdc_l1wvpcr::W](ltdc_l1wvpcr::W) writer structure"]
impl crate::Writable for LTDC_L1WVPCR {}
#[doc = "This register defines the vertical position (first and last line) of the layer1 or 2 window. The first visible line of a frame is the programmed value of AVBP\\[11:0\\]
bits + 1 in the register LTDC_BPCR register. The last visible line of a frame is the programmed value of AAH\\[11:0\\]
bits in the LTDC_AWCR register."]
pub mod ltdc_l1wvpcr;
#[doc = "This register defines the color key value (RGB), that is used by the color keying.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_l1ckcr](ltdc_l1ckcr) module"]
pub type LTDC_L1CKCR = crate::Reg<u32, _LTDC_L1CKCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTDC_L1CKCR;
#[doc = "`read()` method returns [ltdc_l1ckcr::R](ltdc_l1ckcr::R) reader structure"]
impl crate::Readable for LTDC_L1CKCR {}
#[doc = "`write(|w| ..)` method takes [ltdc_l1ckcr::W](ltdc_l1ckcr::W) writer structure"]
impl crate::Writable for LTDC_L1CKCR {}
#[doc = "This register defines the color key value (RGB), that is used by the color keying."]
pub mod ltdc_l1ckcr;
#[doc = "This register defines the pixel format that is used for the stored data in the frame buffer of a layer. The pixel data is read from the frame buffer and then transformed to the internal format 8888 (ARGB).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_l1pfcr](ltdc_l1pfcr) module"]
pub type LTDC_L1PFCR = crate::Reg<u32, _LTDC_L1PFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTDC_L1PFCR;
#[doc = "`read()` method returns [ltdc_l1pfcr::R](ltdc_l1pfcr::R) reader structure"]
impl crate::Readable for LTDC_L1PFCR {}
#[doc = "`write(|w| ..)` method takes [ltdc_l1pfcr::W](ltdc_l1pfcr::W) writer structure"]
impl crate::Writable for LTDC_L1PFCR {}
#[doc = "This register defines the pixel format that is used for the stored data in the frame buffer of a layer. The pixel data is read from the frame buffer and then transformed to the internal format 8888 (ARGB)."]
pub mod ltdc_l1pfcr;
#[doc = "This register defines the constant alpha value (divided by 255 by hardware), that is used in the alpha blending. Refer to LTDC_LxBFCR register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_l1cacr](ltdc_l1cacr) module"]
pub type LTDC_L1CACR = crate::Reg<u32, _LTDC_L1CACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTDC_L1CACR;
#[doc = "`read()` method returns [ltdc_l1cacr::R](ltdc_l1cacr::R) reader structure"]
impl crate::Readable for LTDC_L1CACR {}
#[doc = "`write(|w| ..)` method takes [ltdc_l1cacr::W](ltdc_l1cacr::W) writer structure"]
impl crate::Writable for LTDC_L1CACR {}
#[doc = "This register defines the constant alpha value (divided by 255 by hardware), that is used in the alpha blending. Refer to LTDC_LxBFCR register."]
pub mod ltdc_l1cacr;
#[doc = "This register defines the default color of a layer in the format ARGB. The default color is used outside the defined layer window or when a layer is disabled. The reset value of 0x00000000 defines a transparent black color.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_l1dccr](ltdc_l1dccr) module"]
pub type LTDC_L1DCCR = crate::Reg<u32, _LTDC_L1DCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTDC_L1DCCR;
#[doc = "`read()` method returns [ltdc_l1dccr::R](ltdc_l1dccr::R) reader structure"]
impl crate::Readable for LTDC_L1DCCR {}
#[doc = "`write(|w| ..)` method takes [ltdc_l1dccr::W](ltdc_l1dccr::W) writer structure"]
impl crate::Writable for LTDC_L1DCCR {}
#[doc = "This register defines the default color of a layer in the format ARGB. The default color is used outside the defined layer window or when a layer is disabled. The reset value of 0x00000000 defines a transparent black color."]
pub mod ltdc_l1dccr;
#[doc = "This register defines the blending factors F1 and F2. The general blending formula is: BC = BF1 x C + BF2 x Cs BC = blended color BF1 = blend factor 1 C = current layer color BF2 = blend factor 2 Cs = subjacent layers blended color\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_l1bfcr](ltdc_l1bfcr) module"]
pub type LTDC_L1BFCR = crate::Reg<u32, _LTDC_L1BFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTDC_L1BFCR;
#[doc = "`read()` method returns [ltdc_l1bfcr::R](ltdc_l1bfcr::R) reader structure"]
impl crate::Readable for LTDC_L1BFCR {}
#[doc = "`write(|w| ..)` method takes [ltdc_l1bfcr::W](ltdc_l1bfcr::W) writer structure"]
impl crate::Writable for LTDC_L1BFCR {}
#[doc = "This register defines the blending factors F1 and F2. The general blending formula is: BC = BF1 x C + BF2 x Cs BC = blended color BF1 = blend factor 1 C = current layer color BF2 = blend factor 2 Cs = subjacent layers blended color"]
pub mod ltdc_l1bfcr;
#[doc = "This register defines the color frame buffer start address which has to point to the address where the pixel data of the top left pixel of a layer is stored in the frame buffer.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_l1cfbar](ltdc_l1cfbar) module"]
pub type LTDC_L1CFBAR = crate::Reg<u32, _LTDC_L1CFBAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTDC_L1CFBAR;
#[doc = "`read()` method returns [ltdc_l1cfbar::R](ltdc_l1cfbar::R) reader structure"]
impl crate::Readable for LTDC_L1CFBAR {}
#[doc = "`write(|w| ..)` method takes [ltdc_l1cfbar::W](ltdc_l1cfbar::W) writer structure"]
impl crate::Writable for LTDC_L1CFBAR {}
#[doc = "This register defines the color frame buffer start address which has to point to the address where the pixel data of the top left pixel of a layer is stored in the frame buffer."]
pub mod ltdc_l1cfbar;
#[doc = "This register defines the color frame buffer line length and pitch.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_l1cfblr](ltdc_l1cfblr) module"]
pub type LTDC_L1CFBLR = crate::Reg<u32, _LTDC_L1CFBLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTDC_L1CFBLR;
#[doc = "`read()` method returns [ltdc_l1cfblr::R](ltdc_l1cfblr::R) reader structure"]
impl crate::Readable for LTDC_L1CFBLR {}
#[doc = "`write(|w| ..)` method takes [ltdc_l1cfblr::W](ltdc_l1cfblr::W) writer structure"]
impl crate::Writable for LTDC_L1CFBLR {}
#[doc = "This register defines the color frame buffer line length and pitch."]
pub mod ltdc_l1cfblr;
#[doc = "This register defines the number of lines in the color frame buffer.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_l1cfblnr](ltdc_l1cfblnr) module"]
pub type LTDC_L1CFBLNR = crate::Reg<u32, _LTDC_L1CFBLNR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTDC_L1CFBLNR;
#[doc = "`read()` method returns [ltdc_l1cfblnr::R](ltdc_l1cfblnr::R) reader structure"]
impl crate::Readable for LTDC_L1CFBLNR {}
#[doc = "`write(|w| ..)` method takes [ltdc_l1cfblnr::W](ltdc_l1cfblnr::W) writer structure"]
impl crate::Writable for LTDC_L1CFBLNR {}
#[doc = "This register defines the number of lines in the color frame buffer."]
pub mod ltdc_l1cfblnr;
#[doc = "This register defines the CLUT address and the RGB value.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_l1clutwr](ltdc_l1clutwr) module"]
pub type LTDC_L1CLUTWR = crate::Reg<u32, _LTDC_L1CLUTWR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTDC_L1CLUTWR;
#[doc = "`write(|w| ..)` method takes [ltdc_l1clutwr::W](ltdc_l1clutwr::W) writer structure"]
impl crate::Writable for LTDC_L1CLUTWR {}
#[doc = "This register defines the CLUT address and the RGB value."]
pub mod ltdc_l1clutwr;
#[doc = "LTDC layer 2 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_l2cr](ltdc_l2cr) module"]
pub type LTDC_L2CR = crate::Reg<u32, _LTDC_L2CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTDC_L2CR;
#[doc = "`read()` method returns [ltdc_l2cr::R](ltdc_l2cr::R) reader structure"]
impl crate::Readable for LTDC_L2CR {}
#[doc = "`write(|w| ..)` method takes [ltdc_l2cr::W](ltdc_l2cr::W) writer structure"]
impl crate::Writable for LTDC_L2CR {}
#[doc = "LTDC layer 2 control register"]
pub mod ltdc_l2cr;
#[doc = "This register defines the horizontal position (first and last pixel) of the layer 1 or 2 window. The first visible pixel of a line is the programmed value of AHBP\\[11:0\\]
bits + 1 in the LTDC_BPCR register. The last visible pixel of a line is the programmed value of AAW\\[11:0\\]
bits in the LTDC_AWCR register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_l2whpcr](ltdc_l2whpcr) module"]
pub type LTDC_L2WHPCR = crate::Reg<u32, _LTDC_L2WHPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTDC_L2WHPCR;
#[doc = "`read()` method returns [ltdc_l2whpcr::R](ltdc_l2whpcr::R) reader structure"]
impl crate::Readable for LTDC_L2WHPCR {}
#[doc = "`write(|w| ..)` method takes [ltdc_l2whpcr::W](ltdc_l2whpcr::W) writer structure"]
impl crate::Writable for LTDC_L2WHPCR {}
#[doc = "This register defines the horizontal position (first and last pixel) of the layer 1 or 2 window. The first visible pixel of a line is the programmed value of AHBP\\[11:0\\]
bits + 1 in the LTDC_BPCR register. The last visible pixel of a line is the programmed value of AAW\\[11:0\\]
bits in the LTDC_AWCR register."]
pub mod ltdc_l2whpcr;
#[doc = "This register defines the vertical position (first and last line) of the layer1 or 2 window. The first visible line of a frame is the programmed value of AVBP\\[11:0\\]
bits + 1 in the register LTDC_BPCR register. The last visible line of a frame is the programmed value of AAH\\[11:0\\]
bits in the LTDC_AWCR register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_l2wvpcr](ltdc_l2wvpcr) module"]
pub type LTDC_L2WVPCR = crate::Reg<u32, _LTDC_L2WVPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTDC_L2WVPCR;
#[doc = "`read()` method returns [ltdc_l2wvpcr::R](ltdc_l2wvpcr::R) reader structure"]
impl crate::Readable for LTDC_L2WVPCR {}
#[doc = "`write(|w| ..)` method takes [ltdc_l2wvpcr::W](ltdc_l2wvpcr::W) writer structure"]
impl crate::Writable for LTDC_L2WVPCR {}
#[doc = "This register defines the vertical position (first and last line) of the layer1 or 2 window. The first visible line of a frame is the programmed value of AVBP\\[11:0\\]
bits + 1 in the register LTDC_BPCR register. The last visible line of a frame is the programmed value of AAH\\[11:0\\]
bits in the LTDC_AWCR register."]
pub mod ltdc_l2wvpcr;
#[doc = "This register defines the color key value (RGB), that is used by the color keying.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_l2ckcr](ltdc_l2ckcr) module"]
pub type LTDC_L2CKCR = crate::Reg<u32, _LTDC_L2CKCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTDC_L2CKCR;
#[doc = "`read()` method returns [ltdc_l2ckcr::R](ltdc_l2ckcr::R) reader structure"]
impl crate::Readable for LTDC_L2CKCR {}
#[doc = "`write(|w| ..)` method takes [ltdc_l2ckcr::W](ltdc_l2ckcr::W) writer structure"]
impl crate::Writable for LTDC_L2CKCR {}
#[doc = "This register defines the color key value (RGB), that is used by the color keying."]
pub mod ltdc_l2ckcr;
#[doc = "This register defines the pixel format that is used for the stored data in the frame buffer of a layer. The pixel data is read from the frame buffer and then transformed to the internal format 8888 (ARGB).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_l2pfcr](ltdc_l2pfcr) module"]
pub type LTDC_L2PFCR = crate::Reg<u32, _LTDC_L2PFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTDC_L2PFCR;
#[doc = "`read()` method returns [ltdc_l2pfcr::R](ltdc_l2pfcr::R) reader structure"]
impl crate::Readable for LTDC_L2PFCR {}
#[doc = "`write(|w| ..)` method takes [ltdc_l2pfcr::W](ltdc_l2pfcr::W) writer structure"]
impl crate::Writable for LTDC_L2PFCR {}
#[doc = "This register defines the pixel format that is used for the stored data in the frame buffer of a layer. The pixel data is read from the frame buffer and then transformed to the internal format 8888 (ARGB)."]
pub mod ltdc_l2pfcr;
#[doc = "This register defines the constant alpha value (divided by 255 by hardware), that is used in the alpha blending. Refer to LTDC_LxBFCR register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_l2cacr](ltdc_l2cacr) module"]
pub type LTDC_L2CACR = crate::Reg<u32, _LTDC_L2CACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTDC_L2CACR;
#[doc = "`read()` method returns [ltdc_l2cacr::R](ltdc_l2cacr::R) reader structure"]
impl crate::Readable for LTDC_L2CACR {}
#[doc = "`write(|w| ..)` method takes [ltdc_l2cacr::W](ltdc_l2cacr::W) writer structure"]
impl crate::Writable for LTDC_L2CACR {}
#[doc = "This register defines the constant alpha value (divided by 255 by hardware), that is used in the alpha blending. Refer to LTDC_LxBFCR register."]
pub mod ltdc_l2cacr;
#[doc = "This register defines the default color of a layer in the format ARGB. The default color is used outside the defined layer window or when a layer is disabled. The reset value of 0x00000000 defines a transparent black color.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_l2dccr](ltdc_l2dccr) module"]
pub type LTDC_L2DCCR = crate::Reg<u32, _LTDC_L2DCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTDC_L2DCCR;
#[doc = "`read()` method returns [ltdc_l2dccr::R](ltdc_l2dccr::R) reader structure"]
impl crate::Readable for LTDC_L2DCCR {}
#[doc = "`write(|w| ..)` method takes [ltdc_l2dccr::W](ltdc_l2dccr::W) writer structure"]
impl crate::Writable for LTDC_L2DCCR {}
#[doc = "This register defines the default color of a layer in the format ARGB. The default color is used outside the defined layer window or when a layer is disabled. The reset value of 0x00000000 defines a transparent black color."]
pub mod ltdc_l2dccr;
#[doc = "This register defines the blending factors F1 and F2. The general blending formula is: BC = BF1 x C + BF2 x Cs BC = blended color BF1 = blend factor 1 C = current layer color BF2 = blend factor 2 Cs = subjacent layers blended color\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_l2bfcr](ltdc_l2bfcr) module"]
pub type LTDC_L2BFCR = crate::Reg<u32, _LTDC_L2BFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTDC_L2BFCR;
#[doc = "`read()` method returns [ltdc_l2bfcr::R](ltdc_l2bfcr::R) reader structure"]
impl crate::Readable for LTDC_L2BFCR {}
#[doc = "`write(|w| ..)` method takes [ltdc_l2bfcr::W](ltdc_l2bfcr::W) writer structure"]
impl crate::Writable for LTDC_L2BFCR {}
#[doc = "This register defines the blending factors F1 and F2. The general blending formula is: BC = BF1 x C + BF2 x Cs BC = blended color BF1 = blend factor 1 C = current layer color BF2 = blend factor 2 Cs = subjacent layers blended color"]
pub mod ltdc_l2bfcr;
#[doc = "This register defines the color frame buffer start address which has to point to the address where the pixel data of the top left pixel of a layer is stored in the frame buffer.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_l2cfbar](ltdc_l2cfbar) module"]
pub type LTDC_L2CFBAR = crate::Reg<u32, _LTDC_L2CFBAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTDC_L2CFBAR;
#[doc = "`read()` method returns [ltdc_l2cfbar::R](ltdc_l2cfbar::R) reader structure"]
impl crate::Readable for LTDC_L2CFBAR {}
#[doc = "`write(|w| ..)` method takes [ltdc_l2cfbar::W](ltdc_l2cfbar::W) writer structure"]
impl crate::Writable for LTDC_L2CFBAR {}
#[doc = "This register defines the color frame buffer start address which has to point to the address where the pixel data of the top left pixel of a layer is stored in the frame buffer."]
pub mod ltdc_l2cfbar;
#[doc = "This register defines the color frame buffer line length and pitch.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_l2cfblr](ltdc_l2cfblr) module"]
pub type LTDC_L2CFBLR = crate::Reg<u32, _LTDC_L2CFBLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTDC_L2CFBLR;
#[doc = "`read()` method returns [ltdc_l2cfblr::R](ltdc_l2cfblr::R) reader structure"]
impl crate::Readable for LTDC_L2CFBLR {}
#[doc = "`write(|w| ..)` method takes [ltdc_l2cfblr::W](ltdc_l2cfblr::W) writer structure"]
impl crate::Writable for LTDC_L2CFBLR {}
#[doc = "This register defines the color frame buffer line length and pitch."]
pub mod ltdc_l2cfblr;
#[doc = "This register defines the number of lines in the color frame buffer.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_l2cfblnr](ltdc_l2cfblnr) module"]
pub type LTDC_L2CFBLNR = crate::Reg<u32, _LTDC_L2CFBLNR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTDC_L2CFBLNR;
#[doc = "`read()` method returns [ltdc_l2cfblnr::R](ltdc_l2cfblnr::R) reader structure"]
impl crate::Readable for LTDC_L2CFBLNR {}
#[doc = "`write(|w| ..)` method takes [ltdc_l2cfblnr::W](ltdc_l2cfblnr::W) writer structure"]
impl crate::Writable for LTDC_L2CFBLNR {}
#[doc = "This register defines the number of lines in the color frame buffer."]
pub mod ltdc_l2cfblnr;
#[doc = "This register defines the CLUT address and the RGB value.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_l2clutwr](ltdc_l2clutwr) module"]
pub type LTDC_L2CLUTWR = crate::Reg<u32, _LTDC_L2CLUTWR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTDC_L2CLUTWR;
#[doc = "`write(|w| ..)` method takes [ltdc_l2clutwr::W](ltdc_l2clutwr::W) writer structure"]
impl crate::Writable for LTDC_L2CLUTWR {}
#[doc = "This register defines the CLUT address and the RGB value."]
pub mod ltdc_l2clutwr;
