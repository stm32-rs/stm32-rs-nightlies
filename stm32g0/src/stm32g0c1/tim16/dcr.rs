#[doc = "Register `DCR` reader"]
pub struct R(crate::R<DCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCR` writer"]
pub struct W(crate::W<DCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCR_SPEC>;
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
impl From<crate::W<DCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DMA base address This 5-bit field defines the base-address for DMA transfers (when read/write access are done through the TIMx_DMAR address). DBA is defined as an offset starting from the address of the TIMx_CR1 register. Example: ... Example: Let us consider the following transfer: DBL = 7 transfers and DBA = TIMx_CR1. In this case the transfer is done to/from 7 registers starting from the TIMx_CR1 address.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DBA_A {
    #[doc = "0: TIMx_CR1,"]
    B_0X0 = 0,
    #[doc = "1: TIMx_CR2,"]
    B_0X1 = 1,
    #[doc = "2: TIMx_SMCR,"]
    B_0X2 = 2,
}
impl From<DBA_A> for u8 {
    #[inline(always)]
    fn from(variant: DBA_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DBA` reader - DMA base address This 5-bit field defines the base-address for DMA transfers (when read/write access are done through the TIMx_DMAR address). DBA is defined as an offset starting from the address of the TIMx_CR1 register. Example: ... Example: Let us consider the following transfer: DBL = 7 transfers and DBA = TIMx_CR1. In this case the transfer is done to/from 7 registers starting from the TIMx_CR1 address."]
pub struct DBA_R(crate::FieldReader<u8, DBA_A>);
impl DBA_R {
    pub(crate) fn new(bits: u8) -> Self {
        DBA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DBA_A> {
        match self.bits {
            0 => Some(DBA_A::B_0X0),
            1 => Some(DBA_A::B_0X1),
            2 => Some(DBA_A::B_0X2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == DBA_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == DBA_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == DBA_A::B_0X2
    }
}
impl core::ops::Deref for DBA_R {
    type Target = crate::FieldReader<u8, DBA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBA` writer - DMA base address This 5-bit field defines the base-address for DMA transfers (when read/write access are done through the TIMx_DMAR address). DBA is defined as an offset starting from the address of the TIMx_CR1 register. Example: ... Example: Let us consider the following transfer: DBL = 7 transfers and DBA = TIMx_CR1. In this case the transfer is done to/from 7 registers starting from the TIMx_CR1 address."]
pub struct DBA_W<'a> {
    w: &'a mut W,
}
impl<'a> DBA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBA_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "TIMx_CR1,"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DBA_A::B_0X0)
    }
    #[doc = "TIMx_CR2,"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DBA_A::B_0X1)
    }
    #[doc = "TIMx_SMCR,"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(DBA_A::B_0X2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "DMA burst length This 5-bit field defines the length of DMA transfers (the timer recognizes a burst transfer when a read or a write access is done to the TIMx_DMAR address), i.e. the number of transfers. Transfers can be in half-words or in bytes (see example below). ...\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DBL_A {
    #[doc = "0: 1 transfer,"]
    B_0X0 = 0,
    #[doc = "1: 2 transfers,"]
    B_0X1 = 1,
    #[doc = "2: 3 transfers,"]
    B_0X2 = 2,
    #[doc = "17: 18 transfers."]
    B_0X11 = 17,
}
impl From<DBL_A> for u8 {
    #[inline(always)]
    fn from(variant: DBL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DBL` reader - DMA burst length This 5-bit field defines the length of DMA transfers (the timer recognizes a burst transfer when a read or a write access is done to the TIMx_DMAR address), i.e. the number of transfers. Transfers can be in half-words or in bytes (see example below). ..."]
pub struct DBL_R(crate::FieldReader<u8, DBL_A>);
impl DBL_R {
    pub(crate) fn new(bits: u8) -> Self {
        DBL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DBL_A> {
        match self.bits {
            0 => Some(DBL_A::B_0X0),
            1 => Some(DBL_A::B_0X1),
            2 => Some(DBL_A::B_0X2),
            17 => Some(DBL_A::B_0X11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == DBL_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == DBL_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == DBL_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X11`"]
    #[inline(always)]
    pub fn is_b_0x11(&self) -> bool {
        **self == DBL_A::B_0X11
    }
}
impl core::ops::Deref for DBL_R {
    type Target = crate::FieldReader<u8, DBL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBL` writer - DMA burst length This 5-bit field defines the length of DMA transfers (the timer recognizes a burst transfer when a read or a write access is done to the TIMx_DMAR address), i.e. the number of transfers. Transfers can be in half-words or in bytes (see example below). ..."]
pub struct DBL_W<'a> {
    w: &'a mut W,
}
impl<'a> DBL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 transfer,"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DBL_A::B_0X0)
    }
    #[doc = "2 transfers,"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DBL_A::B_0X1)
    }
    #[doc = "3 transfers,"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(DBL_A::B_0X2)
    }
    #[doc = "18 transfers."]
    #[inline(always)]
    pub fn b_0x11(self) -> &'a mut W {
        self.variant(DBL_A::B_0X11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u32 & 0x1f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - DMA base address This 5-bit field defines the base-address for DMA transfers (when read/write access are done through the TIMx_DMAR address). DBA is defined as an offset starting from the address of the TIMx_CR1 register. Example: ... Example: Let us consider the following transfer: DBL = 7 transfers and DBA = TIMx_CR1. In this case the transfer is done to/from 7 registers starting from the TIMx_CR1 address."]
    #[inline(always)]
    pub fn dba(&self) -> DBA_R {
        DBA_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - DMA burst length This 5-bit field defines the length of DMA transfers (the timer recognizes a burst transfer when a read or a write access is done to the TIMx_DMAR address), i.e. the number of transfers. Transfers can be in half-words or in bytes (see example below). ..."]
    #[inline(always)]
    pub fn dbl(&self) -> DBL_R {
        DBL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DMA base address This 5-bit field defines the base-address for DMA transfers (when read/write access are done through the TIMx_DMAR address). DBA is defined as an offset starting from the address of the TIMx_CR1 register. Example: ... Example: Let us consider the following transfer: DBL = 7 transfers and DBA = TIMx_CR1. In this case the transfer is done to/from 7 registers starting from the TIMx_CR1 address."]
    #[inline(always)]
    pub fn dba(&mut self) -> DBA_W {
        DBA_W { w: self }
    }
    #[doc = "Bits 8:12 - DMA burst length This 5-bit field defines the length of DMA transfers (the timer recognizes a burst transfer when a read or a write access is done to the TIMx_DMAR address), i.e. the number of transfers. Transfers can be in half-words or in bytes (see example below). ..."]
    #[inline(always)]
    pub fn dbl(&mut self) -> DBL_W {
        DBL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcr](index.html) module"]
pub struct DCR_SPEC;
impl crate::RegisterSpec for DCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcr::R](R) reader structure"]
impl crate::Readable for DCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcr::W](W) writer structure"]
impl crate::Writable for DCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCR to value 0"]
impl crate::Resettable for DCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
