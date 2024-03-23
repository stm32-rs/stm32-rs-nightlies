#[doc = "Register `BFCR` reader"]
pub type R = crate::R<BFCRrs>;
#[doc = "Register `BFCR` writer"]
pub type W = crate::W<BFCRrs>;
#[doc = "Blending Factor 2\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BF2 {
    #[doc = "5: BF2 = 1 - constant alpha"]
    Constant = 5,
    #[doc = "7: BF2 = 1 - pixel alpha * constant alpha"]
    Pixel = 7,
}
impl From<BF2> for u8 {
    #[inline(always)]
    fn from(variant: BF2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BF2 {
    type Ux = u8;
}
#[doc = "Field `BF2` reader - Blending Factor 2"]
pub type BF2_R = crate::FieldReader<BF2>;
impl BF2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BF2> {
        match self.bits {
            5 => Some(BF2::Constant),
            7 => Some(BF2::Pixel),
            _ => None,
        }
    }
    #[doc = "BF2 = 1 - constant alpha"]
    #[inline(always)]
    pub fn is_constant(&self) -> bool {
        *self == BF2::Constant
    }
    #[doc = "BF2 = 1 - pixel alpha * constant alpha"]
    #[inline(always)]
    pub fn is_pixel(&self) -> bool {
        *self == BF2::Pixel
    }
}
#[doc = "Field `BF2` writer - Blending Factor 2"]
pub type BF2_W<'a, REG> = crate::FieldWriter<'a, REG, 3, BF2>;
impl<'a, REG> BF2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "BF2 = 1 - constant alpha"]
    #[inline(always)]
    pub fn constant(self) -> &'a mut crate::W<REG> {
        self.variant(BF2::Constant)
    }
    #[doc = "BF2 = 1 - pixel alpha * constant alpha"]
    #[inline(always)]
    pub fn pixel(self) -> &'a mut crate::W<REG> {
        self.variant(BF2::Pixel)
    }
}
#[doc = "Blending Factor 1\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BF1 {
    #[doc = "4: BF1 = constant alpha"]
    Constant = 4,
    #[doc = "6: BF1 = pixel alpha * constant alpha"]
    Pixel = 6,
}
impl From<BF1> for u8 {
    #[inline(always)]
    fn from(variant: BF1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BF1 {
    type Ux = u8;
}
#[doc = "Field `BF1` reader - Blending Factor 1"]
pub type BF1_R = crate::FieldReader<BF1>;
impl BF1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BF1> {
        match self.bits {
            4 => Some(BF1::Constant),
            6 => Some(BF1::Pixel),
            _ => None,
        }
    }
    #[doc = "BF1 = constant alpha"]
    #[inline(always)]
    pub fn is_constant(&self) -> bool {
        *self == BF1::Constant
    }
    #[doc = "BF1 = pixel alpha * constant alpha"]
    #[inline(always)]
    pub fn is_pixel(&self) -> bool {
        *self == BF1::Pixel
    }
}
#[doc = "Field `BF1` writer - Blending Factor 1"]
pub type BF1_W<'a, REG> = crate::FieldWriter<'a, REG, 3, BF1>;
impl<'a, REG> BF1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "BF1 = constant alpha"]
    #[inline(always)]
    pub fn constant(self) -> &'a mut crate::W<REG> {
        self.variant(BF1::Constant)
    }
    #[doc = "BF1 = pixel alpha * constant alpha"]
    #[inline(always)]
    pub fn pixel(self) -> &'a mut crate::W<REG> {
        self.variant(BF1::Pixel)
    }
}
impl R {
    #[doc = "Bits 0:2 - Blending Factor 2"]
    #[inline(always)]
    pub fn bf2(&self) -> BF2_R {
        BF2_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - Blending Factor 1"]
    #[inline(always)]
    pub fn bf1(&self) -> BF1_R {
        BF1_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Blending Factor 2"]
    #[inline(always)]
    #[must_use]
    pub fn bf2(&mut self) -> BF2_W<BFCRrs> {
        BF2_W::new(self, 0)
    }
    #[doc = "Bits 8:10 - Blending Factor 1"]
    #[inline(always)]
    #[must_use]
    pub fn bf1(&mut self) -> BF1_W<BFCRrs> {
        BF1_W::new(self, 8)
    }
}
#[doc = "Layerx Blending Factors Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bfcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bfcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BFCRrs;
impl crate::RegisterSpec for BFCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bfcr::R`](R) reader structure"]
impl crate::Readable for BFCRrs {}
#[doc = "`write(|w| ..)` method takes [`bfcr::W`](W) writer structure"]
impl crate::Writable for BFCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BFCR to value 0x0607"]
impl crate::Resettable for BFCRrs {
    const RESET_VALUE: u32 = 0x0607;
}
