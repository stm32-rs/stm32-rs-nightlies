#[doc = "Register `SDCMR` reader"]
pub struct R(crate::R<SDCMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDCMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDCMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDCMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDCMR` writer"]
pub struct W(crate::W<SDCMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDCMR_SPEC>;
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
impl From<crate::W<SDCMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDCMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE` reader - Command mode These bits define the command issued to the SDRAM device. Note: When a command is issued, at least one Command Target Bank bit ( CTB1 or CTB2) must be set otherwise the command will be ignored. Note: If two SDRAM banks are used, the Auto-refresh and PALL command must be issued simultaneously to the two devices with CTB1 and CTB2 bits set otherwise the command will be ignored. Note: If only one SDRAM bank is used and a command is issued with its associated CTB bit set, the other CTB bit of the unused bank must be kept to 0."]
pub struct MODE_R(crate::FieldReader<u8, u8>);
impl MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE` writer - Command mode These bits define the command issued to the SDRAM device. Note: When a command is issued, at least one Command Target Bank bit ( CTB1 or CTB2) must be set otherwise the command will be ignored. Note: If two SDRAM banks are used, the Auto-refresh and PALL command must be issued simultaneously to the two devices with CTB1 and CTB2 bits set otherwise the command will be ignored. Note: If only one SDRAM bank is used and a command is issued with its associated CTB bit set, the other CTB bit of the unused bank must be kept to 0."]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `CTB2` reader - Command Target Bank 2 This bit indicates whether the command will be issued to SDRAM Bank 2 or not."]
pub struct CTB2_R(crate::FieldReader<bool, bool>);
impl CTB2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTB2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTB2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTB2` writer - Command Target Bank 2 This bit indicates whether the command will be issued to SDRAM Bank 2 or not."]
pub struct CTB2_W<'a> {
    w: &'a mut W,
}
impl<'a> CTB2_W<'a> {
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
#[doc = "Field `CTB1` reader - Command Target Bank 1 This bit indicates whether the command will be issued to SDRAM Bank 1 or not."]
pub struct CTB1_R(crate::FieldReader<bool, bool>);
impl CTB1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTB1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTB1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTB1` writer - Command Target Bank 1 This bit indicates whether the command will be issued to SDRAM Bank 1 or not."]
pub struct CTB1_W<'a> {
    w: &'a mut W,
}
impl<'a> CTB1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `NRFS` reader - Number of Auto-refresh These bits define the number of consecutive Auto-refresh commands issued when MODE = 011. ...."]
pub struct NRFS_R(crate::FieldReader<u8, u8>);
impl NRFS_R {
    pub(crate) fn new(bits: u8) -> Self {
        NRFS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NRFS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NRFS` writer - Number of Auto-refresh These bits define the number of consecutive Auto-refresh commands issued when MODE = 011. ...."]
pub struct NRFS_W<'a> {
    w: &'a mut W,
}
impl<'a> NRFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 5)) | ((value as u32 & 0x0f) << 5);
        self.w
    }
}
#[doc = "Field `MRD` reader - Mode Register definition This 14-bit field defines the SDRAM Mode Register content. The Mode Register is programmed using the Load Mode Register command. The MRD\\[13:0\\]
bits are also used to program the extended mode register for mobile SDRAM."]
pub struct MRD_R(crate::FieldReader<u16, u16>);
impl MRD_R {
    pub(crate) fn new(bits: u16) -> Self {
        MRD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MRD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MRD` writer - Mode Register definition This 14-bit field defines the SDRAM Mode Register content. The Mode Register is programmed using the Load Mode Register command. The MRD\\[13:0\\]
bits are also used to program the extended mode register for mobile SDRAM."]
pub struct MRD_W<'a> {
    w: &'a mut W,
}
impl<'a> MRD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 9)) | ((value as u32 & 0x3fff) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Command mode These bits define the command issued to the SDRAM device. Note: When a command is issued, at least one Command Target Bank bit ( CTB1 or CTB2) must be set otherwise the command will be ignored. Note: If two SDRAM banks are used, the Auto-refresh and PALL command must be issued simultaneously to the two devices with CTB1 and CTB2 bits set otherwise the command will be ignored. Note: If only one SDRAM bank is used and a command is issued with its associated CTB bit set, the other CTB bit of the unused bank must be kept to 0."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Command Target Bank 2 This bit indicates whether the command will be issued to SDRAM Bank 2 or not."]
    #[inline(always)]
    pub fn ctb2(&self) -> CTB2_R {
        CTB2_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Command Target Bank 1 This bit indicates whether the command will be issued to SDRAM Bank 1 or not."]
    #[inline(always)]
    pub fn ctb1(&self) -> CTB1_R {
        CTB1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:8 - Number of Auto-refresh These bits define the number of consecutive Auto-refresh commands issued when MODE = 011. ...."]
    #[inline(always)]
    pub fn nrfs(&self) -> NRFS_R {
        NRFS_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 9:22 - Mode Register definition This 14-bit field defines the SDRAM Mode Register content. The Mode Register is programmed using the Load Mode Register command. The MRD\\[13:0\\]
bits are also used to program the extended mode register for mobile SDRAM."]
    #[inline(always)]
    pub fn mrd(&self) -> MRD_R {
        MRD_R::new(((self.bits >> 9) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - Command mode These bits define the command issued to the SDRAM device. Note: When a command is issued, at least one Command Target Bank bit ( CTB1 or CTB2) must be set otherwise the command will be ignored. Note: If two SDRAM banks are used, the Auto-refresh and PALL command must be issued simultaneously to the two devices with CTB1 and CTB2 bits set otherwise the command will be ignored. Note: If only one SDRAM bank is used and a command is issued with its associated CTB bit set, the other CTB bit of the unused bank must be kept to 0."]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 3 - Command Target Bank 2 This bit indicates whether the command will be issued to SDRAM Bank 2 or not."]
    #[inline(always)]
    pub fn ctb2(&mut self) -> CTB2_W {
        CTB2_W { w: self }
    }
    #[doc = "Bit 4 - Command Target Bank 1 This bit indicates whether the command will be issued to SDRAM Bank 1 or not."]
    #[inline(always)]
    pub fn ctb1(&mut self) -> CTB1_W {
        CTB1_W { w: self }
    }
    #[doc = "Bits 5:8 - Number of Auto-refresh These bits define the number of consecutive Auto-refresh commands issued when MODE = 011. ...."]
    #[inline(always)]
    pub fn nrfs(&mut self) -> NRFS_W {
        NRFS_W { w: self }
    }
    #[doc = "Bits 9:22 - Mode Register definition This 14-bit field defines the SDRAM Mode Register content. The Mode Register is programmed using the Load Mode Register command. The MRD\\[13:0\\]
bits are also used to program the extended mode register for mobile SDRAM."]
    #[inline(always)]
    pub fn mrd(&mut self) -> MRD_W {
        MRD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register contains the command issued when the SDRAM device is accessed. This register is used to initialize the SDRAM device, and to activate the Self-refresh and the Power-down modes. As soon as the MODE field is written, the command will be issued only to one or to both SDRAM banks according to CTB1 and CTB2 command bits. This register is the same for both SDRAM banks.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdcmr](index.html) module"]
pub struct SDCMR_SPEC;
impl crate::RegisterSpec for SDCMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdcmr::R](R) reader structure"]
impl crate::Readable for SDCMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdcmr::W](W) writer structure"]
impl crate::Writable for SDCMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDCMR to value 0"]
impl crate::Resettable for SDCMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
