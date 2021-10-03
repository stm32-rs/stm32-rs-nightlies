#[doc = "Register `OTG_FS_GI2CCTL` reader"]
pub struct R(crate::R<OTG_FS_GI2CCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_FS_GI2CCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_FS_GI2CCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_FS_GI2CCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_FS_GI2CCTL` writer"]
pub struct W(crate::W<OTG_FS_GI2CCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_FS_GI2CCTL_SPEC>;
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
impl From<crate::W<OTG_FS_GI2CCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_FS_GI2CCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RWDATA` reader - I2C Read/Write Data"]
pub struct RWDATA_R(crate::FieldReader<u8, u8>);
impl RWDATA_R {
    pub(crate) fn new(bits: u8) -> Self {
        RWDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RWDATA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWDATA` writer - I2C Read/Write Data"]
pub struct RWDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> RWDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `REGADDR` reader - I2C Register Address"]
pub struct REGADDR_R(crate::FieldReader<u8, u8>);
impl REGADDR_R {
    pub(crate) fn new(bits: u8) -> Self {
        REGADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REGADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGADDR` writer - I2C Register Address"]
pub struct REGADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> REGADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `ADDR` reader - I2C Address"]
pub struct ADDR_R(crate::FieldReader<u8, u8>);
impl ADDR_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDR` writer - I2C Address"]
pub struct ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
#[doc = "Field `I2CEN` reader - I2C Enable"]
pub struct I2CEN_R(crate::FieldReader<bool, bool>);
impl I2CEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2CEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2CEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2CEN` writer - I2C Enable"]
pub struct I2CEN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2CEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `ACK` reader - I2C ACK"]
pub struct ACK_R(crate::FieldReader<bool, bool>);
impl ACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACK` writer - I2C ACK"]
pub struct ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> ACK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `I2CDEVADR` reader - I2C Device Address"]
pub struct I2CDEVADR_R(crate::FieldReader<u8, u8>);
impl I2CDEVADR_R {
    pub(crate) fn new(bits: u8) -> Self {
        I2CDEVADR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2CDEVADR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2CDEVADR` writer - I2C Device Address"]
pub struct I2CDEVADR_W<'a> {
    w: &'a mut W,
}
impl<'a> I2CDEVADR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
#[doc = "Field `I2CDATSE0` reader - I2C DatSe0 USB mode"]
pub struct I2CDATSE0_R(crate::FieldReader<bool, bool>);
impl I2CDATSE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2CDATSE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2CDATSE0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2CDATSE0` writer - I2C DatSe0 USB mode"]
pub struct I2CDATSE0_W<'a> {
    w: &'a mut W,
}
impl<'a> I2CDATSE0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `RW` reader - Read/Write Indicator"]
pub struct RW_R(crate::FieldReader<bool, bool>);
impl RW_R {
    pub(crate) fn new(bits: bool) -> Self {
        RW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RW` writer - Read/Write Indicator"]
pub struct RW_W<'a> {
    w: &'a mut W,
}
impl<'a> RW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `BSYDNE` reader - I2C Busy/Done"]
pub struct BSYDNE_R(crate::FieldReader<bool, bool>);
impl BSYDNE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BSYDNE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BSYDNE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BSYDNE` writer - I2C Busy/Done"]
pub struct BSYDNE_W<'a> {
    w: &'a mut W,
}
impl<'a> BSYDNE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - I2C Read/Write Data"]
    #[inline(always)]
    pub fn rwdata(&self) -> RWDATA_R {
        RWDATA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - I2C Register Address"]
    #[inline(always)]
    pub fn regaddr(&self) -> REGADDR_R {
        REGADDR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:22 - I2C Address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - I2C Enable"]
    #[inline(always)]
    pub fn i2cen(&self) -> I2CEN_R {
        I2CEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - I2C ACK"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 26:27 - I2C Device Address"]
    #[inline(always)]
    pub fn i2cdevadr(&self) -> I2CDEVADR_R {
        I2CDEVADR_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bit 28 - I2C DatSe0 USB mode"]
    #[inline(always)]
    pub fn i2cdatse0(&self) -> I2CDATSE0_R {
        I2CDATSE0_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Read/Write Indicator"]
    #[inline(always)]
    pub fn rw(&self) -> RW_R {
        RW_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - I2C Busy/Done"]
    #[inline(always)]
    pub fn bsydne(&self) -> BSYDNE_R {
        BSYDNE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - I2C Read/Write Data"]
    #[inline(always)]
    pub fn rwdata(&mut self) -> RWDATA_W {
        RWDATA_W { w: self }
    }
    #[doc = "Bits 8:15 - I2C Register Address"]
    #[inline(always)]
    pub fn regaddr(&mut self) -> REGADDR_W {
        REGADDR_W { w: self }
    }
    #[doc = "Bits 16:22 - I2C Address"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W { w: self }
    }
    #[doc = "Bit 23 - I2C Enable"]
    #[inline(always)]
    pub fn i2cen(&mut self) -> I2CEN_W {
        I2CEN_W { w: self }
    }
    #[doc = "Bit 24 - I2C ACK"]
    #[inline(always)]
    pub fn ack(&mut self) -> ACK_W {
        ACK_W { w: self }
    }
    #[doc = "Bits 26:27 - I2C Device Address"]
    #[inline(always)]
    pub fn i2cdevadr(&mut self) -> I2CDEVADR_W {
        I2CDEVADR_W { w: self }
    }
    #[doc = "Bit 28 - I2C DatSe0 USB mode"]
    #[inline(always)]
    pub fn i2cdatse0(&mut self) -> I2CDATSE0_W {
        I2CDATSE0_W { w: self }
    }
    #[doc = "Bit 30 - Read/Write Indicator"]
    #[inline(always)]
    pub fn rw(&mut self) -> RW_W {
        RW_W { w: self }
    }
    #[doc = "Bit 31 - I2C Busy/Done"]
    #[inline(always)]
    pub fn bsydne(&mut self) -> BSYDNE_W {
        BSYDNE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG I2C access register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_gi2cctl](index.html) module"]
pub struct OTG_FS_GI2CCTL_SPEC;
impl crate::RegisterSpec for OTG_FS_GI2CCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_fs_gi2cctl::R](R) reader structure"]
impl crate::Readable for OTG_FS_GI2CCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_fs_gi2cctl::W](W) writer structure"]
impl crate::Writable for OTG_FS_GI2CCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTG_FS_GI2CCTL to value 0x0200_0400"]
impl crate::Resettable for OTG_FS_GI2CCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200_0400
    }
}
