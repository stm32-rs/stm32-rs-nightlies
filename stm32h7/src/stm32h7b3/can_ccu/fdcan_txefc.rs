#[doc = "Register `FDCAN_TXEFC` reader"]
pub type R = crate::R<FDCAN_TXEFCrs>;
#[doc = "Register `FDCAN_TXEFC` writer"]
pub type W = crate::W<FDCAN_TXEFCrs>;
#[doc = "Field `EFSA` reader - Event FIFO Start Address"]
pub type EFSA_R = crate::FieldReader<u16>;
#[doc = "Field `EFSA` writer - Event FIFO Start Address"]
pub type EFSA_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `EFS` reader - Event FIFO Size"]
pub type EFS_R = crate::FieldReader;
#[doc = "Field `EFS` writer - Event FIFO Size"]
pub type EFS_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `EFWM` reader - Event FIFO Watermark"]
pub type EFWM_R = crate::FieldReader;
#[doc = "Field `EFWM` writer - Event FIFO Watermark"]
pub type EFWM_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 2:15 - Event FIFO Start Address"]
    #[inline(always)]
    pub fn efsa(&self) -> EFSA_R {
        EFSA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:21 - Event FIFO Size"]
    #[inline(always)]
    pub fn efs(&self) -> EFS_R {
        EFS_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Event FIFO Watermark"]
    #[inline(always)]
    pub fn efwm(&self) -> EFWM_R {
        EFWM_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 2:15 - Event FIFO Start Address"]
    #[inline(always)]
    #[must_use]
    pub fn efsa(&mut self) -> EFSA_W<FDCAN_TXEFCrs> {
        EFSA_W::new(self, 2)
    }
    #[doc = "Bits 16:21 - Event FIFO Size"]
    #[inline(always)]
    #[must_use]
    pub fn efs(&mut self) -> EFS_W<FDCAN_TXEFCrs> {
        EFS_W::new(self, 16)
    }
    #[doc = "Bits 24:29 - Event FIFO Watermark"]
    #[inline(always)]
    #[must_use]
    pub fn efwm(&mut self) -> EFWM_W<FDCAN_TXEFCrs> {
        EFWM_W::new(self, 24)
    }
}
#[doc = "FDCAN Tx Event FIFO Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txefc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_txefc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_TXEFCrs;
impl crate::RegisterSpec for FDCAN_TXEFCrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_txefc::R`](R) reader structure"]
impl crate::Readable for FDCAN_TXEFCrs {}
#[doc = "`write(|w| ..)` method takes [`fdcan_txefc::W`](W) writer structure"]
impl crate::Writable for FDCAN_TXEFCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_TXEFC to value 0"]
impl crate::Resettable for FDCAN_TXEFCrs {
    const RESET_VALUE: u32 = 0;
}
