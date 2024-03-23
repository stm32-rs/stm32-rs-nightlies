#[doc = "Register `FDCAN_ILS` reader"]
pub type R = crate::R<FDCAN_ILSrs>;
#[doc = "Register `FDCAN_ILS` writer"]
pub type W = crate::W<FDCAN_ILSrs>;
#[doc = "Field `RxFIFO0` reader - RxFIFO0"]
pub type RX_FIFO0_R = crate::BitReader;
#[doc = "Field `RxFIFO0` writer - RxFIFO0"]
pub type RX_FIFO0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RxFIFO1` reader - RxFIFO1"]
pub type RX_FIFO1_R = crate::BitReader;
#[doc = "Field `RxFIFO1` writer - RxFIFO1"]
pub type RX_FIFO1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMSG` reader - SMSG"]
pub type SMSG_R = crate::BitReader;
#[doc = "Field `SMSG` writer - SMSG"]
pub type SMSG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFERR` reader - TFERR"]
pub type TFERR_R = crate::BitReader;
#[doc = "Field `TFERR` writer - TFERR"]
pub type TFERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MISC` reader - MISC"]
pub type MISC_R = crate::BitReader;
#[doc = "Field `MISC` writer - MISC"]
pub type MISC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BERR` reader - BERR"]
pub type BERR_R = crate::BitReader;
#[doc = "Field `BERR` writer - BERR"]
pub type BERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERR` reader - PERR"]
pub type PERR_R = crate::BitReader;
#[doc = "Field `PERR` writer - PERR"]
pub type PERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RxFIFO0"]
    #[inline(always)]
    pub fn rx_fifo0(&self) -> RX_FIFO0_R {
        RX_FIFO0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RxFIFO1"]
    #[inline(always)]
    pub fn rx_fifo1(&self) -> RX_FIFO1_R {
        RX_FIFO1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SMSG"]
    #[inline(always)]
    pub fn smsg(&self) -> SMSG_R {
        SMSG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TFERR"]
    #[inline(always)]
    pub fn tferr(&self) -> TFERR_R {
        TFERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MISC"]
    #[inline(always)]
    pub fn misc(&self) -> MISC_R {
        MISC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - BERR"]
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PERR"]
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RxFIFO0"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo0(&mut self) -> RX_FIFO0_W<FDCAN_ILSrs> {
        RX_FIFO0_W::new(self, 0)
    }
    #[doc = "Bit 1 - RxFIFO1"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo1(&mut self) -> RX_FIFO1_W<FDCAN_ILSrs> {
        RX_FIFO1_W::new(self, 1)
    }
    #[doc = "Bit 2 - SMSG"]
    #[inline(always)]
    #[must_use]
    pub fn smsg(&mut self) -> SMSG_W<FDCAN_ILSrs> {
        SMSG_W::new(self, 2)
    }
    #[doc = "Bit 3 - TFERR"]
    #[inline(always)]
    #[must_use]
    pub fn tferr(&mut self) -> TFERR_W<FDCAN_ILSrs> {
        TFERR_W::new(self, 3)
    }
    #[doc = "Bit 4 - MISC"]
    #[inline(always)]
    #[must_use]
    pub fn misc(&mut self) -> MISC_W<FDCAN_ILSrs> {
        MISC_W::new(self, 4)
    }
    #[doc = "Bit 5 - BERR"]
    #[inline(always)]
    #[must_use]
    pub fn berr(&mut self) -> BERR_W<FDCAN_ILSrs> {
        BERR_W::new(self, 5)
    }
    #[doc = "Bit 6 - PERR"]
    #[inline(always)]
    #[must_use]
    pub fn perr(&mut self) -> PERR_W<FDCAN_ILSrs> {
        PERR_W::new(self, 6)
    }
}
#[doc = "FDCAN Interrupt Line Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ils::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ils::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_ILSrs;
impl crate::RegisterSpec for FDCAN_ILSrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_ils::R`](R) reader structure"]
impl crate::Readable for FDCAN_ILSrs {}
#[doc = "`write(|w| ..)` method takes [`fdcan_ils::W`](W) writer structure"]
impl crate::Writable for FDCAN_ILSrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_ILS to value 0"]
impl crate::Resettable for FDCAN_ILSrs {
    const RESET_VALUE: u32 = 0;
}
