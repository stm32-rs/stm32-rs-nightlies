#[doc = "Register `SDCR` reader"]
pub struct R(crate::R<SDCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDCR` writer"]
pub struct W(crate::W<SDCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDCR_SPEC>;
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
impl From<crate::W<SDCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NC` reader - Number of column address bits These bits define the number of bits of a column address."]
pub struct NC_R(crate::FieldReader<u8, u8>);
impl NC_R {
    pub(crate) fn new(bits: u8) -> Self {
        NC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NC` writer - Number of column address bits These bits define the number of bits of a column address."]
pub struct NC_W<'a> {
    w: &'a mut W,
}
impl<'a> NC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `NR` reader - Number of row address bits These bits define the number of bits of a row address."]
pub struct NR_R(crate::FieldReader<u8, u8>);
impl NR_R {
    pub(crate) fn new(bits: u8) -> Self {
        NR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NR` writer - Number of row address bits These bits define the number of bits of a row address."]
pub struct NR_W<'a> {
    w: &'a mut W,
}
impl<'a> NR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `MWID` reader - Memory data bus width. These bits define the memory device width."]
pub struct MWID_R(crate::FieldReader<u8, u8>);
impl MWID_R {
    pub(crate) fn new(bits: u8) -> Self {
        MWID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MWID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MWID` writer - Memory data bus width. These bits define the memory device width."]
pub struct MWID_W<'a> {
    w: &'a mut W,
}
impl<'a> MWID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `NB` reader - Number of internal banks This bit sets the number of internal banks."]
pub struct NB_R(crate::FieldReader<bool, bool>);
impl NB_R {
    pub(crate) fn new(bits: bool) -> Self {
        NB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NB` writer - Number of internal banks This bit sets the number of internal banks."]
pub struct NB_W<'a> {
    w: &'a mut W,
}
impl<'a> NB_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `CAS` reader - CAS Latency This bits sets the SDRAM CAS latency in number of memory clock cycles"]
pub struct CAS_R(crate::FieldReader<u8, u8>);
impl CAS_R {
    pub(crate) fn new(bits: u8) -> Self {
        CAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAS` writer - CAS Latency This bits sets the SDRAM CAS latency in number of memory clock cycles"]
pub struct CAS_W<'a> {
    w: &'a mut W,
}
impl<'a> CAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 7)) | ((value as u32 & 0x03) << 7);
        self.w
    }
}
#[doc = "Field `WP` reader - Write protection This bit enables write mode access to the SDRAM bank."]
pub struct WP_R(crate::FieldReader<bool, bool>);
impl WP_R {
    pub(crate) fn new(bits: bool) -> Self {
        WP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WP` writer - Write protection This bit enables write mode access to the SDRAM bank."]
pub struct WP_W<'a> {
    w: &'a mut W,
}
impl<'a> WP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `SDCLK` reader - SDRAM clock configuration These bits define the SDRAM clock period for both SDRAM banks and allow disabling the clock before changing the frequency. In this case the SDRAM must be re-initialized. Note: The corresponding bits in the FMC_SDCR2 register is read only."]
pub struct SDCLK_R(crate::FieldReader<u8, u8>);
impl SDCLK_R {
    pub(crate) fn new(bits: u8) -> Self {
        SDCLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDCLK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDCLK` writer - SDRAM clock configuration These bits define the SDRAM clock period for both SDRAM banks and allow disabling the clock before changing the frequency. In this case the SDRAM must be re-initialized. Note: The corresponding bits in the FMC_SDCR2 register is read only."]
pub struct SDCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> SDCLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `RBURST` reader - Burst read This bit enables burst read mode. The SDRAM controller anticipates the next read commands during the CAS latency and stores data in the Read FIFO. Note: The corresponding bit in the FMC_SDCR2 register is read only."]
pub struct RBURST_R(crate::FieldReader<bool, bool>);
impl RBURST_R {
    pub(crate) fn new(bits: bool) -> Self {
        RBURST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBURST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RBURST` writer - Burst read This bit enables burst read mode. The SDRAM controller anticipates the next read commands during the CAS latency and stores data in the Read FIFO. Note: The corresponding bit in the FMC_SDCR2 register is read only."]
pub struct RBURST_W<'a> {
    w: &'a mut W,
}
impl<'a> RBURST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `RPIPE` reader - Read pipe These bits define the delay, in KCK_FMC clock cycles, for reading data after CAS latency. Note: The corresponding bits in the FMC_SDCR2 register is read only."]
pub struct RPIPE_R(crate::FieldReader<u8, u8>);
impl RPIPE_R {
    pub(crate) fn new(bits: u8) -> Self {
        RPIPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPIPE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPIPE` writer - Read pipe These bits define the delay, in KCK_FMC clock cycles, for reading data after CAS latency. Note: The corresponding bits in the FMC_SDCR2 register is read only."]
pub struct RPIPE_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | ((value as u32 & 0x03) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Number of column address bits These bits define the number of bits of a column address."]
    #[inline(always)]
    pub fn nc(&self) -> NC_R {
        NC_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Number of row address bits These bits define the number of bits of a row address."]
    #[inline(always)]
    pub fn nr(&self) -> NR_R {
        NR_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Memory data bus width. These bits define the memory device width."]
    #[inline(always)]
    pub fn mwid(&self) -> MWID_R {
        MWID_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Number of internal banks This bit sets the number of internal banks."]
    #[inline(always)]
    pub fn nb(&self) -> NB_R {
        NB_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 7:8 - CAS Latency This bits sets the SDRAM CAS latency in number of memory clock cycles"]
    #[inline(always)]
    pub fn cas(&self) -> CAS_R {
        CAS_R::new(((self.bits >> 7) & 0x03) as u8)
    }
    #[doc = "Bit 9 - Write protection This bit enables write mode access to the SDRAM bank."]
    #[inline(always)]
    pub fn wp(&self) -> WP_R {
        WP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - SDRAM clock configuration These bits define the SDRAM clock period for both SDRAM banks and allow disabling the clock before changing the frequency. In this case the SDRAM must be re-initialized. Note: The corresponding bits in the FMC_SDCR2 register is read only."]
    #[inline(always)]
    pub fn sdclk(&self) -> SDCLK_R {
        SDCLK_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 12 - Burst read This bit enables burst read mode. The SDRAM controller anticipates the next read commands during the CAS latency and stores data in the Read FIFO. Note: The corresponding bit in the FMC_SDCR2 register is read only."]
    #[inline(always)]
    pub fn rburst(&self) -> RBURST_R {
        RBURST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:14 - Read pipe These bits define the delay, in KCK_FMC clock cycles, for reading data after CAS latency. Note: The corresponding bits in the FMC_SDCR2 register is read only."]
    #[inline(always)]
    pub fn rpipe(&self) -> RPIPE_R {
        RPIPE_R::new(((self.bits >> 13) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Number of column address bits These bits define the number of bits of a column address."]
    #[inline(always)]
    pub fn nc(&mut self) -> NC_W {
        NC_W { w: self }
    }
    #[doc = "Bits 2:3 - Number of row address bits These bits define the number of bits of a row address."]
    #[inline(always)]
    pub fn nr(&mut self) -> NR_W {
        NR_W { w: self }
    }
    #[doc = "Bits 4:5 - Memory data bus width. These bits define the memory device width."]
    #[inline(always)]
    pub fn mwid(&mut self) -> MWID_W {
        MWID_W { w: self }
    }
    #[doc = "Bit 6 - Number of internal banks This bit sets the number of internal banks."]
    #[inline(always)]
    pub fn nb(&mut self) -> NB_W {
        NB_W { w: self }
    }
    #[doc = "Bits 7:8 - CAS Latency This bits sets the SDRAM CAS latency in number of memory clock cycles"]
    #[inline(always)]
    pub fn cas(&mut self) -> CAS_W {
        CAS_W { w: self }
    }
    #[doc = "Bit 9 - Write protection This bit enables write mode access to the SDRAM bank."]
    #[inline(always)]
    pub fn wp(&mut self) -> WP_W {
        WP_W { w: self }
    }
    #[doc = "Bits 10:11 - SDRAM clock configuration These bits define the SDRAM clock period for both SDRAM banks and allow disabling the clock before changing the frequency. In this case the SDRAM must be re-initialized. Note: The corresponding bits in the FMC_SDCR2 register is read only."]
    #[inline(always)]
    pub fn sdclk(&mut self) -> SDCLK_W {
        SDCLK_W { w: self }
    }
    #[doc = "Bit 12 - Burst read This bit enables burst read mode. The SDRAM controller anticipates the next read commands during the CAS latency and stores data in the Read FIFO. Note: The corresponding bit in the FMC_SDCR2 register is read only."]
    #[inline(always)]
    pub fn rburst(&mut self) -> RBURST_W {
        RBURST_W { w: self }
    }
    #[doc = "Bits 13:14 - Read pipe These bits define the delay, in KCK_FMC clock cycles, for reading data after CAS latency. Note: The corresponding bits in the FMC_SDCR2 register is read only."]
    #[inline(always)]
    pub fn rpipe(&mut self) -> RPIPE_W {
        RPIPE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register contains the control parameters for each SDRAM memory bank\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdcr](index.html) module"]
pub struct SDCR_SPEC;
impl crate::RegisterSpec for SDCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdcr::R](R) reader structure"]
impl crate::Readable for SDCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdcr::W](W) writer structure"]
impl crate::Writable for SDCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDCR to value 0x02d0"]
impl crate::Resettable for SDCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02d0
    }
}
