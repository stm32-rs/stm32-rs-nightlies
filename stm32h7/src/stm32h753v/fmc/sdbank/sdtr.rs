#[doc = "Register `SDTR` reader"]
pub struct R(crate::R<SDTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDTR` writer"]
pub struct W(crate::W<SDTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SDTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TMRD` reader - Load Mode Register to Active These bits define the delay between a Load Mode Register command and an Active or Refresh command in number of memory clock cycles. ...."]
pub struct TMRD_R(crate::FieldReader<u8, u8>);
impl TMRD_R {
    pub(crate) fn new(bits: u8) -> Self {
        TMRD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMRD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRD` writer - Load Mode Register to Active These bits define the delay between a Load Mode Register command and an Active or Refresh command in number of memory clock cycles. ...."]
pub struct TMRD_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `TXSR` reader - Exit Self-refresh delay These bits define the delay from releasing the Self-refresh command to issuing the Activate command in number of memory clock cycles. .... Note: If two SDRAM devices are used, the FMC_SDTR1 and FMC_SDTR2 must be programmed with the same TXSR timing corresponding to the slowest SDRAM device."]
pub struct TXSR_R(crate::FieldReader<u8, u8>);
impl TXSR_R {
    pub(crate) fn new(bits: u8) -> Self {
        TXSR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXSR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXSR` writer - Exit Self-refresh delay These bits define the delay from releasing the Self-refresh command to issuing the Activate command in number of memory clock cycles. .... Note: If two SDRAM devices are used, the FMC_SDTR1 and FMC_SDTR2 must be programmed with the same TXSR timing corresponding to the slowest SDRAM device."]
pub struct TXSR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `TRAS` reader - Self refresh time These bits define the minimum Self-refresh period in number of memory clock cycles. ...."]
pub struct TRAS_R(crate::FieldReader<u8, u8>);
impl TRAS_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRAS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRAS` writer - Self refresh time These bits define the minimum Self-refresh period in number of memory clock cycles. ...."]
pub struct TRAS_W<'a> {
    w: &'a mut W,
}
impl<'a> TRAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `TRC` reader - Row cycle delay These bits define the delay between the Refresh command and the Activate command, as well as the delay between two consecutive Refresh commands. It is expressed in number of memory clock cycles. The TRC timing is only configured in the FMC_SDTR1 register. If two SDRAM devices are used, the TRC must be programmed with the timings of the slowest device. .... Note: TRC must match the TRC and TRFC (Auto Refresh period) timings defined in the SDRAM device datasheet. Note: The corresponding bits in the FMC_SDTR2 register are dont care."]
pub struct TRC_R(crate::FieldReader<u8, u8>);
impl TRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRC` writer - Row cycle delay These bits define the delay between the Refresh command and the Activate command, as well as the delay between two consecutive Refresh commands. It is expressed in number of memory clock cycles. The TRC timing is only configured in the FMC_SDTR1 register. If two SDRAM devices are used, the TRC must be programmed with the timings of the slowest device. .... Note: TRC must match the TRC and TRFC (Auto Refresh period) timings defined in the SDRAM device datasheet. Note: The corresponding bits in the FMC_SDTR2 register are dont care."]
pub struct TRC_W<'a> {
    w: &'a mut W,
}
impl<'a> TRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `TWR` reader - Recovery delay These bits define the delay between a Write and a Precharge command in number of memory clock cycles. .... Note: TWR must be programmed to match the write recovery time (tWR) defined in the SDRAM datasheet, and to guarantee that: TWR &#8805; TRAS - TRCD and TWR &#8805;TRC - TRCD - TRP Example: TRAS= 4 cycles, TRCD= 2 cycles. So, TWR &gt;= 2 cycles. TWR must be programmed to 0x1. If two SDRAM devices are used, the FMC_SDTR1 and FMC_SDTR2 must be programmed with the same TWR timing corresponding to the slowest SDRAM device."]
pub struct TWR_R(crate::FieldReader<u8, u8>);
impl TWR_R {
    pub(crate) fn new(bits: u8) -> Self {
        TWR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TWR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TWR` writer - Recovery delay These bits define the delay between a Write and a Precharge command in number of memory clock cycles. .... Note: TWR must be programmed to match the write recovery time (tWR) defined in the SDRAM datasheet, and to guarantee that: TWR &#8805; TRAS - TRCD and TWR &#8805;TRC - TRCD - TRP Example: TRAS= 4 cycles, TRCD= 2 cycles. So, TWR &gt;= 2 cycles. TWR must be programmed to 0x1. If two SDRAM devices are used, the FMC_SDTR1 and FMC_SDTR2 must be programmed with the same TWR timing corresponding to the slowest SDRAM device."]
pub struct TWR_W<'a> {
    w: &'a mut W,
}
impl<'a> TWR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `TRP` reader - Row precharge delay These bits define the delay between a Precharge command and another command in number of memory clock cycles. The TRP timing is only configured in the FMC_SDTR1 register. If two SDRAM devices are used, the TRP must be programmed with the timing of the slowest device. .... Note: The corresponding bits in the FMC_SDTR2 register are dont care."]
pub struct TRP_R(crate::FieldReader<u8, u8>);
impl TRP_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRP` writer - Row precharge delay These bits define the delay between a Precharge command and another command in number of memory clock cycles. The TRP timing is only configured in the FMC_SDTR1 register. If two SDRAM devices are used, the TRP must be programmed with the timing of the slowest device. .... Note: The corresponding bits in the FMC_SDTR2 register are dont care."]
pub struct TRP_W<'a> {
    w: &'a mut W,
}
impl<'a> TRP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `TRCD` reader - Row to column delay These bits define the delay between the Activate command and a Read/Write command in number of memory clock cycles. ...."]
pub struct TRCD_R(crate::FieldReader<u8, u8>);
impl TRCD_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRCD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRCD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRCD` writer - Row to column delay These bits define the delay between the Activate command and a Read/Write command in number of memory clock cycles. ...."]
pub struct TRCD_W<'a> {
    w: &'a mut W,
}
impl<'a> TRCD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Load Mode Register to Active These bits define the delay between a Load Mode Register command and an Active or Refresh command in number of memory clock cycles. ...."]
    #[inline(always)]
    pub fn tmrd(&self) -> TMRD_R {
        TMRD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Exit Self-refresh delay These bits define the delay from releasing the Self-refresh command to issuing the Activate command in number of memory clock cycles. .... Note: If two SDRAM devices are used, the FMC_SDTR1 and FMC_SDTR2 must be programmed with the same TXSR timing corresponding to the slowest SDRAM device."]
    #[inline(always)]
    pub fn txsr(&self) -> TXSR_R {
        TXSR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Self refresh time These bits define the minimum Self-refresh period in number of memory clock cycles. ...."]
    #[inline(always)]
    pub fn tras(&self) -> TRAS_R {
        TRAS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Row cycle delay These bits define the delay between the Refresh command and the Activate command, as well as the delay between two consecutive Refresh commands. It is expressed in number of memory clock cycles. The TRC timing is only configured in the FMC_SDTR1 register. If two SDRAM devices are used, the TRC must be programmed with the timings of the slowest device. .... Note: TRC must match the TRC and TRFC (Auto Refresh period) timings defined in the SDRAM device datasheet. Note: The corresponding bits in the FMC_SDTR2 register are dont care."]
    #[inline(always)]
    pub fn trc(&self) -> TRC_R {
        TRC_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Recovery delay These bits define the delay between a Write and a Precharge command in number of memory clock cycles. .... Note: TWR must be programmed to match the write recovery time (tWR) defined in the SDRAM datasheet, and to guarantee that: TWR &#8805; TRAS - TRCD and TWR &#8805;TRC - TRCD - TRP Example: TRAS= 4 cycles, TRCD= 2 cycles. So, TWR &gt;= 2 cycles. TWR must be programmed to 0x1. If two SDRAM devices are used, the FMC_SDTR1 and FMC_SDTR2 must be programmed with the same TWR timing corresponding to the slowest SDRAM device."]
    #[inline(always)]
    pub fn twr(&self) -> TWR_R {
        TWR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Row precharge delay These bits define the delay between a Precharge command and another command in number of memory clock cycles. The TRP timing is only configured in the FMC_SDTR1 register. If two SDRAM devices are used, the TRP must be programmed with the timing of the slowest device. .... Note: The corresponding bits in the FMC_SDTR2 register are dont care."]
    #[inline(always)]
    pub fn trp(&self) -> TRP_R {
        TRP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Row to column delay These bits define the delay between the Activate command and a Read/Write command in number of memory clock cycles. ...."]
    #[inline(always)]
    pub fn trcd(&self) -> TRCD_R {
        TRCD_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Load Mode Register to Active These bits define the delay between a Load Mode Register command and an Active or Refresh command in number of memory clock cycles. ...."]
    #[inline(always)]
    pub fn tmrd(&mut self) -> TMRD_W {
        TMRD_W { w: self }
    }
    #[doc = "Bits 4:7 - Exit Self-refresh delay These bits define the delay from releasing the Self-refresh command to issuing the Activate command in number of memory clock cycles. .... Note: If two SDRAM devices are used, the FMC_SDTR1 and FMC_SDTR2 must be programmed with the same TXSR timing corresponding to the slowest SDRAM device."]
    #[inline(always)]
    pub fn txsr(&mut self) -> TXSR_W {
        TXSR_W { w: self }
    }
    #[doc = "Bits 8:11 - Self refresh time These bits define the minimum Self-refresh period in number of memory clock cycles. ...."]
    #[inline(always)]
    pub fn tras(&mut self) -> TRAS_W {
        TRAS_W { w: self }
    }
    #[doc = "Bits 12:15 - Row cycle delay These bits define the delay between the Refresh command and the Activate command, as well as the delay between two consecutive Refresh commands. It is expressed in number of memory clock cycles. The TRC timing is only configured in the FMC_SDTR1 register. If two SDRAM devices are used, the TRC must be programmed with the timings of the slowest device. .... Note: TRC must match the TRC and TRFC (Auto Refresh period) timings defined in the SDRAM device datasheet. Note: The corresponding bits in the FMC_SDTR2 register are dont care."]
    #[inline(always)]
    pub fn trc(&mut self) -> TRC_W {
        TRC_W { w: self }
    }
    #[doc = "Bits 16:19 - Recovery delay These bits define the delay between a Write and a Precharge command in number of memory clock cycles. .... Note: TWR must be programmed to match the write recovery time (tWR) defined in the SDRAM datasheet, and to guarantee that: TWR &#8805; TRAS - TRCD and TWR &#8805;TRC - TRCD - TRP Example: TRAS= 4 cycles, TRCD= 2 cycles. So, TWR &gt;= 2 cycles. TWR must be programmed to 0x1. If two SDRAM devices are used, the FMC_SDTR1 and FMC_SDTR2 must be programmed with the same TWR timing corresponding to the slowest SDRAM device."]
    #[inline(always)]
    pub fn twr(&mut self) -> TWR_W {
        TWR_W { w: self }
    }
    #[doc = "Bits 20:23 - Row precharge delay These bits define the delay between a Precharge command and another command in number of memory clock cycles. The TRP timing is only configured in the FMC_SDTR1 register. If two SDRAM devices are used, the TRP must be programmed with the timing of the slowest device. .... Note: The corresponding bits in the FMC_SDTR2 register are dont care."]
    #[inline(always)]
    pub fn trp(&mut self) -> TRP_W {
        TRP_W { w: self }
    }
    #[doc = "Bits 24:27 - Row to column delay These bits define the delay between the Activate command and a Read/Write command in number of memory clock cycles. ...."]
    #[inline(always)]
    pub fn trcd(&mut self) -> TRCD_W {
        TRCD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register contains the timing parameters of each SDRAM bank\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdtr](index.html) module"]
pub struct SDTR_SPEC;
impl crate::RegisterSpec for SDTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdtr::R](R) reader structure"]
impl crate::Readable for SDTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdtr::W](W) writer structure"]
impl crate::Writable for SDTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDTR to value 0x0fff_ffff"]
impl crate::Resettable for SDTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0fff_ffff
    }
}
