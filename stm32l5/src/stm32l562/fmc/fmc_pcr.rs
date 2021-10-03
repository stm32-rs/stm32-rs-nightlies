#[doc = "Register `FMC_PCR` reader"]
pub struct R(crate::R<FMC_PCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_PCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_PCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_PCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMC_PCR` writer"]
pub struct W(crate::W<FMC_PCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_PCR_SPEC>;
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
impl From<crate::W<FMC_PCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMC_PCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWAITEN` reader - Wait feature enable bit. This bit enables the Wait feature for the NAND Flash memory bank:"]
pub struct PWAITEN_R(crate::FieldReader<bool, bool>);
impl PWAITEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWAITEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWAITEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWAITEN` writer - Wait feature enable bit. This bit enables the Wait feature for the NAND Flash memory bank:"]
pub struct PWAITEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWAITEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `PBKEN` reader - NAND Flash memory bank enable bit. This bit enables the memory bank. Accessing a disabled memory bank causes an ERROR on AXI bus"]
pub struct PBKEN_R(crate::FieldReader<bool, bool>);
impl PBKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PBKEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PBKEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBKEN` writer - NAND Flash memory bank enable bit. This bit enables the memory bank. Accessing a disabled memory bank causes an ERROR on AXI bus"]
pub struct PBKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PBKEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `PTYP` reader - Memory type"]
pub struct PTYP_R(crate::FieldReader<bool, bool>);
impl PTYP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTYP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PTYP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTYP` writer - Memory type"]
pub struct PTYP_W<'a> {
    w: &'a mut W,
}
impl<'a> PTYP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `PWID` reader - Data bus width. These bits define the external memory device width."]
pub struct PWID_R(crate::FieldReader<u8, u8>);
impl PWID_R {
    pub(crate) fn new(bits: u8) -> Self {
        PWID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWID` writer - Data bus width. These bits define the external memory device width."]
pub struct PWID_W<'a> {
    w: &'a mut W,
}
impl<'a> PWID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `ECCEN` reader - ECC computation logic enable bit"]
pub struct ECCEN_R(crate::FieldReader<bool, bool>);
impl ECCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ECCEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECCEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECCEN` writer - ECC computation logic enable bit"]
pub struct ECCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCEN_W<'a> {
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
#[doc = "Field `TCLR` reader - CLE to RE delay. These bits set time from CLE low to RE low in number of KCK_FMC clock cycles. The time is give by the following formula: t_clr = (TCLR + SET + 2) TKCK_FMC where TKCK_FMC is the KCK_FMC clock period Note: Set is MEMSET or ATTSET according to the addressed space."]
pub struct TCLR_R(crate::FieldReader<u8, u8>);
impl TCLR_R {
    pub(crate) fn new(bits: u8) -> Self {
        TCLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCLR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCLR` writer - CLE to RE delay. These bits set time from CLE low to RE low in number of KCK_FMC clock cycles. The time is give by the following formula: t_clr = (TCLR + SET + 2) TKCK_FMC where TKCK_FMC is the KCK_FMC clock period Note: Set is MEMSET or ATTSET according to the addressed space."]
pub struct TCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TCLR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 9)) | ((value as u32 & 0x0f) << 9);
        self.w
    }
}
#[doc = "Field `TAR` reader - ALE to RE delay. These bits set time from ALE low to RE low in number of KCK_FMC clock cycles. Time is: t_ar = (TAR + SET + 2) TKCK_FMC where TKCK_FMC is the FMC clock period Note: Set is MEMSET or ATTSET according to the addressed space."]
pub struct TAR_R(crate::FieldReader<u8, u8>);
impl TAR_R {
    pub(crate) fn new(bits: u8) -> Self {
        TAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAR` writer - ALE to RE delay. These bits set time from ALE low to RE low in number of KCK_FMC clock cycles. Time is: t_ar = (TAR + SET + 2) TKCK_FMC where TKCK_FMC is the FMC clock period Note: Set is MEMSET or ATTSET according to the addressed space."]
pub struct TAR_W<'a> {
    w: &'a mut W,
}
impl<'a> TAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 13)) | ((value as u32 & 0x0f) << 13);
        self.w
    }
}
#[doc = "Field `ECCPS` reader - ECC page size. These bits define the page size for the extended ECC:"]
pub struct ECCPS_R(crate::FieldReader<u8, u8>);
impl ECCPS_R {
    pub(crate) fn new(bits: u8) -> Self {
        ECCPS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECCPS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECCPS` writer - ECC page size. These bits define the page size for the extended ECC:"]
pub struct ECCPS_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCPS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 17)) | ((value as u32 & 0x07) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Wait feature enable bit. This bit enables the Wait feature for the NAND Flash memory bank:"]
    #[inline(always)]
    pub fn pwaiten(&self) -> PWAITEN_R {
        PWAITEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - NAND Flash memory bank enable bit. This bit enables the memory bank. Accessing a disabled memory bank causes an ERROR on AXI bus"]
    #[inline(always)]
    pub fn pbken(&self) -> PBKEN_R {
        PBKEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Memory type"]
    #[inline(always)]
    pub fn ptyp(&self) -> PTYP_R {
        PTYP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Data bus width. These bits define the external memory device width."]
    #[inline(always)]
    pub fn pwid(&self) -> PWID_R {
        PWID_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - ECC computation logic enable bit"]
    #[inline(always)]
    pub fn eccen(&self) -> ECCEN_R {
        ECCEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 9:12 - CLE to RE delay. These bits set time from CLE low to RE low in number of KCK_FMC clock cycles. The time is give by the following formula: t_clr = (TCLR + SET + 2) TKCK_FMC where TKCK_FMC is the KCK_FMC clock period Note: Set is MEMSET or ATTSET according to the addressed space."]
    #[inline(always)]
    pub fn tclr(&self) -> TCLR_R {
        TCLR_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 13:16 - ALE to RE delay. These bits set time from ALE low to RE low in number of KCK_FMC clock cycles. Time is: t_ar = (TAR + SET + 2) TKCK_FMC where TKCK_FMC is the FMC clock period Note: Set is MEMSET or ATTSET according to the addressed space."]
    #[inline(always)]
    pub fn tar(&self) -> TAR_R {
        TAR_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bits 17:19 - ECC page size. These bits define the page size for the extended ECC:"]
    #[inline(always)]
    pub fn eccps(&self) -> ECCPS_R {
        ECCPS_R::new(((self.bits >> 17) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Wait feature enable bit. This bit enables the Wait feature for the NAND Flash memory bank:"]
    #[inline(always)]
    pub fn pwaiten(&mut self) -> PWAITEN_W {
        PWAITEN_W { w: self }
    }
    #[doc = "Bit 2 - NAND Flash memory bank enable bit. This bit enables the memory bank. Accessing a disabled memory bank causes an ERROR on AXI bus"]
    #[inline(always)]
    pub fn pbken(&mut self) -> PBKEN_W {
        PBKEN_W { w: self }
    }
    #[doc = "Bit 3 - Memory type"]
    #[inline(always)]
    pub fn ptyp(&mut self) -> PTYP_W {
        PTYP_W { w: self }
    }
    #[doc = "Bits 4:5 - Data bus width. These bits define the external memory device width."]
    #[inline(always)]
    pub fn pwid(&mut self) -> PWID_W {
        PWID_W { w: self }
    }
    #[doc = "Bit 6 - ECC computation logic enable bit"]
    #[inline(always)]
    pub fn eccen(&mut self) -> ECCEN_W {
        ECCEN_W { w: self }
    }
    #[doc = "Bits 9:12 - CLE to RE delay. These bits set time from CLE low to RE low in number of KCK_FMC clock cycles. The time is give by the following formula: t_clr = (TCLR + SET + 2) TKCK_FMC where TKCK_FMC is the KCK_FMC clock period Note: Set is MEMSET or ATTSET according to the addressed space."]
    #[inline(always)]
    pub fn tclr(&mut self) -> TCLR_W {
        TCLR_W { w: self }
    }
    #[doc = "Bits 13:16 - ALE to RE delay. These bits set time from ALE low to RE low in number of KCK_FMC clock cycles. Time is: t_ar = (TAR + SET + 2) TKCK_FMC where TKCK_FMC is the FMC clock period Note: Set is MEMSET or ATTSET according to the addressed space."]
    #[inline(always)]
    pub fn tar(&mut self) -> TAR_W {
        TAR_W { w: self }
    }
    #[doc = "Bits 17:19 - ECC page size. These bits define the page size for the extended ECC:"]
    #[inline(always)]
    pub fn eccps(&mut self) -> ECCPS_W {
        ECCPS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "NAND Flash control registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_pcr](index.html) module"]
pub struct FMC_PCR_SPEC;
impl crate::RegisterSpec for FMC_PCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_pcr::R](R) reader structure"]
impl crate::Readable for FMC_PCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmc_pcr::W](W) writer structure"]
impl crate::Writable for FMC_PCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FMC_PCR to value 0x18"]
impl crate::Resettable for FMC_PCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x18
    }
}
