#[doc = "Register `ETH_MACL3L4C1R` reader"]
pub type R = crate::R<ETH_MACL3L4C1Rrs>;
#[doc = "Register `ETH_MACL3L4C1R` writer"]
pub type W = crate::W<ETH_MACL3L4C1Rrs>;
#[doc = "Field `L3PEN1` reader - L3PEN1"]
pub type L3PEN1_R = crate::BitReader;
#[doc = "Field `L3PEN1` writer - L3PEN1"]
pub type L3PEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L3SAM1` reader - L3SAM1"]
pub type L3SAM1_R = crate::BitReader;
#[doc = "Field `L3SAM1` writer - L3SAM1"]
pub type L3SAM1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L3SAIM1` reader - L3SAIM1"]
pub type L3SAIM1_R = crate::BitReader;
#[doc = "Field `L3SAIM1` writer - L3SAIM1"]
pub type L3SAIM1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L3DAM1` reader - L3DAM1"]
pub type L3DAM1_R = crate::BitReader;
#[doc = "Field `L3DAM1` writer - L3DAM1"]
pub type L3DAM1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L3DAIM1` reader - L3DAIM1"]
pub type L3DAIM1_R = crate::BitReader;
#[doc = "Field `L3DAIM1` writer - L3DAIM1"]
pub type L3DAIM1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L3HSBM1` reader - L3HSBM1"]
pub type L3HSBM1_R = crate::FieldReader;
#[doc = "Field `L3HSBM1` writer - L3HSBM1"]
pub type L3HSBM1_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `L3HDBM1` reader - L3HDBM1"]
pub type L3HDBM1_R = crate::FieldReader;
#[doc = "Field `L3HDBM1` writer - L3HDBM1"]
pub type L3HDBM1_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `L4PEN1` reader - L4PEN1"]
pub type L4PEN1_R = crate::BitReader;
#[doc = "Field `L4PEN1` writer - L4PEN1"]
pub type L4PEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L4SPM1` reader - L4SPM1"]
pub type L4SPM1_R = crate::BitReader;
#[doc = "Field `L4SPM1` writer - L4SPM1"]
pub type L4SPM1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L4SPIM1` reader - L4SPIM1"]
pub type L4SPIM1_R = crate::BitReader;
#[doc = "Field `L4SPIM1` writer - L4SPIM1"]
pub type L4SPIM1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L4DPM1` reader - L4DPM1"]
pub type L4DPM1_R = crate::BitReader;
#[doc = "Field `L4DPM1` writer - L4DPM1"]
pub type L4DPM1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L4DPIM1` reader - L4DPIM1"]
pub type L4DPIM1_R = crate::BitReader;
#[doc = "Field `L4DPIM1` writer - L4DPIM1"]
pub type L4DPIM1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - L3PEN1"]
    #[inline(always)]
    pub fn l3pen1(&self) -> L3PEN1_R {
        L3PEN1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - L3SAM1"]
    #[inline(always)]
    pub fn l3sam1(&self) -> L3SAM1_R {
        L3SAM1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - L3SAIM1"]
    #[inline(always)]
    pub fn l3saim1(&self) -> L3SAIM1_R {
        L3SAIM1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - L3DAM1"]
    #[inline(always)]
    pub fn l3dam1(&self) -> L3DAM1_R {
        L3DAM1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - L3DAIM1"]
    #[inline(always)]
    pub fn l3daim1(&self) -> L3DAIM1_R {
        L3DAIM1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:10 - L3HSBM1"]
    #[inline(always)]
    pub fn l3hsbm1(&self) -> L3HSBM1_R {
        L3HSBM1_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - L3HDBM1"]
    #[inline(always)]
    pub fn l3hdbm1(&self) -> L3HDBM1_R {
        L3HDBM1_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - L4PEN1"]
    #[inline(always)]
    pub fn l4pen1(&self) -> L4PEN1_R {
        L4PEN1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - L4SPM1"]
    #[inline(always)]
    pub fn l4spm1(&self) -> L4SPM1_R {
        L4SPM1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - L4SPIM1"]
    #[inline(always)]
    pub fn l4spim1(&self) -> L4SPIM1_R {
        L4SPIM1_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - L4DPM1"]
    #[inline(always)]
    pub fn l4dpm1(&self) -> L4DPM1_R {
        L4DPM1_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - L4DPIM1"]
    #[inline(always)]
    pub fn l4dpim1(&self) -> L4DPIM1_R {
        L4DPIM1_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - L3PEN1"]
    #[inline(always)]
    #[must_use]
    pub fn l3pen1(&mut self) -> L3PEN1_W<ETH_MACL3L4C1Rrs> {
        L3PEN1_W::new(self, 0)
    }
    #[doc = "Bit 2 - L3SAM1"]
    #[inline(always)]
    #[must_use]
    pub fn l3sam1(&mut self) -> L3SAM1_W<ETH_MACL3L4C1Rrs> {
        L3SAM1_W::new(self, 2)
    }
    #[doc = "Bit 3 - L3SAIM1"]
    #[inline(always)]
    #[must_use]
    pub fn l3saim1(&mut self) -> L3SAIM1_W<ETH_MACL3L4C1Rrs> {
        L3SAIM1_W::new(self, 3)
    }
    #[doc = "Bit 4 - L3DAM1"]
    #[inline(always)]
    #[must_use]
    pub fn l3dam1(&mut self) -> L3DAM1_W<ETH_MACL3L4C1Rrs> {
        L3DAM1_W::new(self, 4)
    }
    #[doc = "Bit 5 - L3DAIM1"]
    #[inline(always)]
    #[must_use]
    pub fn l3daim1(&mut self) -> L3DAIM1_W<ETH_MACL3L4C1Rrs> {
        L3DAIM1_W::new(self, 5)
    }
    #[doc = "Bits 6:10 - L3HSBM1"]
    #[inline(always)]
    #[must_use]
    pub fn l3hsbm1(&mut self) -> L3HSBM1_W<ETH_MACL3L4C1Rrs> {
        L3HSBM1_W::new(self, 6)
    }
    #[doc = "Bits 11:15 - L3HDBM1"]
    #[inline(always)]
    #[must_use]
    pub fn l3hdbm1(&mut self) -> L3HDBM1_W<ETH_MACL3L4C1Rrs> {
        L3HDBM1_W::new(self, 11)
    }
    #[doc = "Bit 16 - L4PEN1"]
    #[inline(always)]
    #[must_use]
    pub fn l4pen1(&mut self) -> L4PEN1_W<ETH_MACL3L4C1Rrs> {
        L4PEN1_W::new(self, 16)
    }
    #[doc = "Bit 18 - L4SPM1"]
    #[inline(always)]
    #[must_use]
    pub fn l4spm1(&mut self) -> L4SPM1_W<ETH_MACL3L4C1Rrs> {
        L4SPM1_W::new(self, 18)
    }
    #[doc = "Bit 19 - L4SPIM1"]
    #[inline(always)]
    #[must_use]
    pub fn l4spim1(&mut self) -> L4SPIM1_W<ETH_MACL3L4C1Rrs> {
        L4SPIM1_W::new(self, 19)
    }
    #[doc = "Bit 20 - L4DPM1"]
    #[inline(always)]
    #[must_use]
    pub fn l4dpm1(&mut self) -> L4DPM1_W<ETH_MACL3L4C1Rrs> {
        L4DPM1_W::new(self, 20)
    }
    #[doc = "Bit 21 - L4DPIM1"]
    #[inline(always)]
    #[must_use]
    pub fn l4dpim1(&mut self) -> L4DPIM1_W<ETH_MACL3L4C1Rrs> {
        L4DPIM1_W::new(self, 21)
    }
}
#[doc = "The Layer 3 and Layer 4 Control register controls the operations of filter 0 of Layer 3 and Layer 4.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macl3l4c1r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macl3l4c1r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MACL3L4C1Rrs;
impl crate::RegisterSpec for ETH_MACL3L4C1Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_macl3l4c1r::R`](R) reader structure"]
impl crate::Readable for ETH_MACL3L4C1Rrs {}
#[doc = "`write(|w| ..)` method takes [`eth_macl3l4c1r::W`](W) writer structure"]
impl crate::Writable for ETH_MACL3L4C1Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_MACL3L4C1R to value 0"]
impl crate::Resettable for ETH_MACL3L4C1Rrs {
    const RESET_VALUE: u32 = 0;
}
