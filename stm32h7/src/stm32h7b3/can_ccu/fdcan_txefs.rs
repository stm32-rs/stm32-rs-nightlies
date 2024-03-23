#[doc = "Register `FDCAN_TXEFS` reader"]
pub type R = crate::R<FDCAN_TXEFSrs>;
#[doc = "Register `FDCAN_TXEFS` writer"]
pub type W = crate::W<FDCAN_TXEFSrs>;
#[doc = "Field `EFFL` reader - Event FIFO Fill Level"]
pub type EFFL_R = crate::FieldReader;
#[doc = "Field `EFFL` writer - Event FIFO Fill Level"]
pub type EFFL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `EFGI` reader - Event FIFO Get Index."]
pub type EFGI_R = crate::FieldReader;
#[doc = "Field `EFGI` writer - Event FIFO Get Index."]
pub type EFGI_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `EFF` reader - Event FIFO Full."]
pub type EFF_R = crate::BitReader;
#[doc = "Field `EFF` writer - Event FIFO Full."]
pub type EFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEFL` reader - Tx Event FIFO Element Lost."]
pub type TEFL_R = crate::BitReader;
#[doc = "Field `TEFL` writer - Tx Event FIFO Element Lost."]
pub type TEFL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Event FIFO Fill Level"]
    #[inline(always)]
    pub fn effl(&self) -> EFFL_R {
        EFFL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:12 - Event FIFO Get Index."]
    #[inline(always)]
    pub fn efgi(&self) -> EFGI_R {
        EFGI_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - Event FIFO Full."]
    #[inline(always)]
    pub fn eff(&self) -> EFF_R {
        EFF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Tx Event FIFO Element Lost."]
    #[inline(always)]
    pub fn tefl(&self) -> TEFL_R {
        TEFL_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Event FIFO Fill Level"]
    #[inline(always)]
    #[must_use]
    pub fn effl(&mut self) -> EFFL_W<FDCAN_TXEFSrs> {
        EFFL_W::new(self, 0)
    }
    #[doc = "Bits 8:12 - Event FIFO Get Index."]
    #[inline(always)]
    #[must_use]
    pub fn efgi(&mut self) -> EFGI_W<FDCAN_TXEFSrs> {
        EFGI_W::new(self, 8)
    }
    #[doc = "Bit 24 - Event FIFO Full."]
    #[inline(always)]
    #[must_use]
    pub fn eff(&mut self) -> EFF_W<FDCAN_TXEFSrs> {
        EFF_W::new(self, 24)
    }
    #[doc = "Bit 25 - Tx Event FIFO Element Lost."]
    #[inline(always)]
    #[must_use]
    pub fn tefl(&mut self) -> TEFL_W<FDCAN_TXEFSrs> {
        TEFL_W::new(self, 25)
    }
}
#[doc = "FDCAN Tx Event FIFO Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txefs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_txefs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_TXEFSrs;
impl crate::RegisterSpec for FDCAN_TXEFSrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_txefs::R`](R) reader structure"]
impl crate::Readable for FDCAN_TXEFSrs {}
#[doc = "`write(|w| ..)` method takes [`fdcan_txefs::W`](W) writer structure"]
impl crate::Writable for FDCAN_TXEFSrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_TXEFS to value 0"]
impl crate::Resettable for FDCAN_TXEFSrs {
    const RESET_VALUE: u32 = 0;
}
