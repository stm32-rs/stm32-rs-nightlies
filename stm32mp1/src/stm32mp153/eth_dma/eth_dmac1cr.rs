#[doc = "Register `ETH_DMAC1CR` reader"]
pub type R = crate::R<ETH_DMAC1CRrs>;
#[doc = "Register `ETH_DMAC1CR` writer"]
pub type W = crate::W<ETH_DMAC1CRrs>;
#[doc = "Field `MSS` reader - MSS"]
pub type MSS_R = crate::FieldReader<u16>;
#[doc = "Field `MSS` writer - MSS"]
pub type MSS_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `PBLX8` reader - PBLX8"]
pub type PBLX8_R = crate::BitReader;
#[doc = "Field `PBLX8` writer - PBLX8"]
pub type PBLX8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSL` reader - DSL"]
pub type DSL_R = crate::FieldReader;
#[doc = "Field `DSL` writer - DSL"]
pub type DSL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:13 - MSS"]
    #[inline(always)]
    pub fn mss(&self) -> MSS_R {
        MSS_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 16 - PBLX8"]
    #[inline(always)]
    pub fn pblx8(&self) -> PBLX8_R {
        PBLX8_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 18:20 - DSL"]
    #[inline(always)]
    pub fn dsl(&self) -> DSL_R {
        DSL_R::new(((self.bits >> 18) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:13 - MSS"]
    #[inline(always)]
    #[must_use]
    pub fn mss(&mut self) -> MSS_W<ETH_DMAC1CRrs> {
        MSS_W::new(self, 0)
    }
    #[doc = "Bit 16 - PBLX8"]
    #[inline(always)]
    #[must_use]
    pub fn pblx8(&mut self) -> PBLX8_W<ETH_DMAC1CRrs> {
        PBLX8_W::new(self, 16)
    }
    #[doc = "Bits 18:20 - DSL"]
    #[inline(always)]
    #[must_use]
    pub fn dsl(&mut self) -> DSL_W<ETH_DMAC1CRrs> {
        DSL_W::new(self, 18)
    }
}
#[doc = "Channel 1 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_dmac1cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_dmac1cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_DMAC1CRrs;
impl crate::RegisterSpec for ETH_DMAC1CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_dmac1cr::R`](R) reader structure"]
impl crate::Readable for ETH_DMAC1CRrs {}
#[doc = "`write(|w| ..)` method takes [`eth_dmac1cr::W`](W) writer structure"]
impl crate::Writable for ETH_DMAC1CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_DMAC1CR to value 0"]
impl crate::Resettable for ETH_DMAC1CRrs {
    const RESET_VALUE: u32 = 0;
}
