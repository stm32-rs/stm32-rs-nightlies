#[doc = "Register `IFCR` reader"]
pub type R = crate::R<IFCRrs>;
#[doc = "Register `IFCR` writer"]
pub type W = crate::W<IFCRrs>;
#[doc = "Clear Transfer error interrupt flag Programming this bit to 1 clears the TEIF flag in the DMA2D_ISR register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTEIF {
    #[doc = "1: Clear the TEIF flag in the ISR register"]
    Clear = 1,
}
impl From<CTEIF> for bool {
    #[inline(always)]
    fn from(variant: CTEIF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTEIF` reader - Clear Transfer error interrupt flag Programming this bit to 1 clears the TEIF flag in the DMA2D_ISR register"]
pub type CTEIF_R = crate::BitReader<CTEIF>;
impl CTEIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CTEIF> {
        match self.bits {
            true => Some(CTEIF::Clear),
            _ => None,
        }
    }
    #[doc = "Clear the TEIF flag in the ISR register"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CTEIF::Clear
    }
}
#[doc = "Field `CTEIF` writer - Clear Transfer error interrupt flag Programming this bit to 1 clears the TEIF flag in the DMA2D_ISR register"]
pub type CTEIF_W<'a, REG> = crate::BitWriter<'a, REG, CTEIF>;
impl<'a, REG> CTEIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the TEIF flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CTEIF::Clear)
    }
}
#[doc = "Clear transfer complete interrupt flag Programming this bit to 1 clears the TCIF flag in the DMA2D_ISR register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTCIF {
    #[doc = "1: Clear the TCIF flag in the ISR register"]
    Clear = 1,
}
impl From<CTCIF> for bool {
    #[inline(always)]
    fn from(variant: CTCIF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTCIF` reader - Clear transfer complete interrupt flag Programming this bit to 1 clears the TCIF flag in the DMA2D_ISR register"]
pub type CTCIF_R = crate::BitReader<CTCIF>;
impl CTCIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CTCIF> {
        match self.bits {
            true => Some(CTCIF::Clear),
            _ => None,
        }
    }
    #[doc = "Clear the TCIF flag in the ISR register"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CTCIF::Clear
    }
}
#[doc = "Field `CTCIF` writer - Clear transfer complete interrupt flag Programming this bit to 1 clears the TCIF flag in the DMA2D_ISR register"]
pub type CTCIF_W<'a, REG> = crate::BitWriter<'a, REG, CTCIF>;
impl<'a, REG> CTCIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the TCIF flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CTCIF::Clear)
    }
}
#[doc = "Clear transfer watermark interrupt flag Programming this bit to 1 clears the TWIF flag in the DMA2D_ISR register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTWIF {
    #[doc = "1: Clear the TWIF flag in the ISR register"]
    Clear = 1,
}
impl From<CTWIF> for bool {
    #[inline(always)]
    fn from(variant: CTWIF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTWIF` reader - Clear transfer watermark interrupt flag Programming this bit to 1 clears the TWIF flag in the DMA2D_ISR register"]
pub type CTWIF_R = crate::BitReader<CTWIF>;
impl CTWIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CTWIF> {
        match self.bits {
            true => Some(CTWIF::Clear),
            _ => None,
        }
    }
    #[doc = "Clear the TWIF flag in the ISR register"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CTWIF::Clear
    }
}
#[doc = "Field `CTWIF` writer - Clear transfer watermark interrupt flag Programming this bit to 1 clears the TWIF flag in the DMA2D_ISR register"]
pub type CTWIF_W<'a, REG> = crate::BitWriter<'a, REG, CTWIF>;
impl<'a, REG> CTWIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the TWIF flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CTWIF::Clear)
    }
}
#[doc = "Clear CLUT access error interrupt flag Programming this bit to 1 clears the CAEIF flag in the DMA2D_ISR register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAECIF {
    #[doc = "1: Clear the CAEIF flag in the ISR register"]
    Clear = 1,
}
impl From<CAECIF> for bool {
    #[inline(always)]
    fn from(variant: CAECIF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAECIF` reader - Clear CLUT access error interrupt flag Programming this bit to 1 clears the CAEIF flag in the DMA2D_ISR register"]
pub type CAECIF_R = crate::BitReader<CAECIF>;
impl CAECIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CAECIF> {
        match self.bits {
            true => Some(CAECIF::Clear),
            _ => None,
        }
    }
    #[doc = "Clear the CAEIF flag in the ISR register"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CAECIF::Clear
    }
}
#[doc = "Field `CAECIF` writer - Clear CLUT access error interrupt flag Programming this bit to 1 clears the CAEIF flag in the DMA2D_ISR register"]
pub type CAECIF_W<'a, REG> = crate::BitWriter<'a, REG, CAECIF>;
impl<'a, REG> CAECIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the CAEIF flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CAECIF::Clear)
    }
}
#[doc = "Clear CLUT transfer complete interrupt flag Programming this bit to 1 clears the CTCIF flag in the DMA2D_ISR register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCTCIF {
    #[doc = "1: Clear the CTCIF flag in the ISR register"]
    Clear = 1,
}
impl From<CCTCIF> for bool {
    #[inline(always)]
    fn from(variant: CCTCIF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCTCIF` reader - Clear CLUT transfer complete interrupt flag Programming this bit to 1 clears the CTCIF flag in the DMA2D_ISR register"]
pub type CCTCIF_R = crate::BitReader<CCTCIF>;
impl CCTCIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CCTCIF> {
        match self.bits {
            true => Some(CCTCIF::Clear),
            _ => None,
        }
    }
    #[doc = "Clear the CTCIF flag in the ISR register"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CCTCIF::Clear
    }
}
#[doc = "Field `CCTCIF` writer - Clear CLUT transfer complete interrupt flag Programming this bit to 1 clears the CTCIF flag in the DMA2D_ISR register"]
pub type CCTCIF_W<'a, REG> = crate::BitWriter<'a, REG, CCTCIF>;
impl<'a, REG> CCTCIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the CTCIF flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CCTCIF::Clear)
    }
}
#[doc = "Clear configuration error interrupt flag Programming this bit to 1 clears the CEIF flag in the DMA2D_ISR register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCEIF {
    #[doc = "1: Clear the CEIF flag in the ISR register"]
    Clear = 1,
}
impl From<CCEIF> for bool {
    #[inline(always)]
    fn from(variant: CCEIF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCEIF` reader - Clear configuration error interrupt flag Programming this bit to 1 clears the CEIF flag in the DMA2D_ISR register"]
pub type CCEIF_R = crate::BitReader<CCEIF>;
impl CCEIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CCEIF> {
        match self.bits {
            true => Some(CCEIF::Clear),
            _ => None,
        }
    }
    #[doc = "Clear the CEIF flag in the ISR register"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CCEIF::Clear
    }
}
#[doc = "Field `CCEIF` writer - Clear configuration error interrupt flag Programming this bit to 1 clears the CEIF flag in the DMA2D_ISR register"]
pub type CCEIF_W<'a, REG> = crate::BitWriter<'a, REG, CCEIF>;
impl<'a, REG> CCEIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the CEIF flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CCEIF::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - Clear Transfer error interrupt flag Programming this bit to 1 clears the TEIF flag in the DMA2D_ISR register"]
    #[inline(always)]
    pub fn cteif(&self) -> CTEIF_R {
        CTEIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear transfer complete interrupt flag Programming this bit to 1 clears the TCIF flag in the DMA2D_ISR register"]
    #[inline(always)]
    pub fn ctcif(&self) -> CTCIF_R {
        CTCIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear transfer watermark interrupt flag Programming this bit to 1 clears the TWIF flag in the DMA2D_ISR register"]
    #[inline(always)]
    pub fn ctwif(&self) -> CTWIF_R {
        CTWIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear CLUT access error interrupt flag Programming this bit to 1 clears the CAEIF flag in the DMA2D_ISR register"]
    #[inline(always)]
    pub fn caecif(&self) -> CAECIF_R {
        CAECIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clear CLUT transfer complete interrupt flag Programming this bit to 1 clears the CTCIF flag in the DMA2D_ISR register"]
    #[inline(always)]
    pub fn cctcif(&self) -> CCTCIF_R {
        CCTCIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Clear configuration error interrupt flag Programming this bit to 1 clears the CEIF flag in the DMA2D_ISR register"]
    #[inline(always)]
    pub fn cceif(&self) -> CCEIF_R {
        CCEIF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear Transfer error interrupt flag Programming this bit to 1 clears the TEIF flag in the DMA2D_ISR register"]
    #[inline(always)]
    #[must_use]
    pub fn cteif(&mut self) -> CTEIF_W<IFCRrs> {
        CTEIF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear transfer complete interrupt flag Programming this bit to 1 clears the TCIF flag in the DMA2D_ISR register"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif(&mut self) -> CTCIF_W<IFCRrs> {
        CTCIF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear transfer watermark interrupt flag Programming this bit to 1 clears the TWIF flag in the DMA2D_ISR register"]
    #[inline(always)]
    #[must_use]
    pub fn ctwif(&mut self) -> CTWIF_W<IFCRrs> {
        CTWIF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear CLUT access error interrupt flag Programming this bit to 1 clears the CAEIF flag in the DMA2D_ISR register"]
    #[inline(always)]
    #[must_use]
    pub fn caecif(&mut self) -> CAECIF_W<IFCRrs> {
        CAECIF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear CLUT transfer complete interrupt flag Programming this bit to 1 clears the CTCIF flag in the DMA2D_ISR register"]
    #[inline(always)]
    #[must_use]
    pub fn cctcif(&mut self) -> CCTCIF_W<IFCRrs> {
        CCTCIF_W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear configuration error interrupt flag Programming this bit to 1 clears the CEIF flag in the DMA2D_ISR register"]
    #[inline(always)]
    #[must_use]
    pub fn cceif(&mut self) -> CCEIF_W<IFCRrs> {
        CCEIF_W::new(self, 5)
    }
}
#[doc = "DMA2D interrupt flag clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ifcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFCRrs;
impl crate::RegisterSpec for IFCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ifcr::R`](R) reader structure"]
impl crate::Readable for IFCRrs {}
#[doc = "`write(|w| ..)` method takes [`ifcr::W`](W) writer structure"]
impl crate::Writable for IFCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFCR to value 0"]
impl crate::Resettable for IFCRrs {
    const RESET_VALUE: u32 = 0;
}
