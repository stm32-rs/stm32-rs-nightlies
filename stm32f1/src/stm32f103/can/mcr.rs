///Register `MCR` reader
pub type R = crate::R<MCRrs>;
///Register `MCR` writer
pub type W = crate::W<MCRrs>;
///Field `INRQ` reader - INRQ
pub type INRQ_R = crate::BitReader;
///Field `INRQ` writer - INRQ
pub type INRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLEEP` reader - SLEEP
pub type SLEEP_R = crate::BitReader;
///Field `SLEEP` writer - SLEEP
pub type SLEEP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXFP` reader - TXFP
pub type TXFP_R = crate::BitReader;
///Field `TXFP` writer - TXFP
pub type TXFP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RFLM` reader - RFLM
pub type RFLM_R = crate::BitReader;
///Field `RFLM` writer - RFLM
pub type RFLM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NART` reader - NART
pub type NART_R = crate::BitReader;
///Field `NART` writer - NART
pub type NART_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AWUM` reader - AWUM
pub type AWUM_R = crate::BitReader;
///Field `AWUM` writer - AWUM
pub type AWUM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ABOM` reader - ABOM
pub type ABOM_R = crate::BitReader;
///Field `ABOM` writer - ABOM
pub type ABOM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TTCM` reader - TTCM
pub type TTCM_R = crate::BitReader;
///Field `TTCM` writer - TTCM
pub type TTCM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RESET` reader - RESET
pub type RESET_R = crate::BitReader;
///Field `RESET` writer - RESET
pub type RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBF` reader - DBF
pub type DBF_R = crate::BitReader;
///Field `DBF` writer - DBF
pub type DBF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - INRQ
    #[inline(always)]
    pub fn inrq(&self) -> INRQ_R {
        INRQ_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SLEEP
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TXFP
    #[inline(always)]
    pub fn txfp(&self) -> TXFP_R {
        TXFP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - RFLM
    #[inline(always)]
    pub fn rflm(&self) -> RFLM_R {
        RFLM_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - NART
    #[inline(always)]
    pub fn nart(&self) -> NART_R {
        NART_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - AWUM
    #[inline(always)]
    pub fn awum(&self) -> AWUM_R {
        AWUM_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - ABOM
    #[inline(always)]
    pub fn abom(&self) -> ABOM_R {
        ABOM_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TTCM
    #[inline(always)]
    pub fn ttcm(&self) -> TTCM_R {
        TTCM_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 15 - RESET
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - DBF
    #[inline(always)]
    pub fn dbf(&self) -> DBF_R {
        DBF_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCR")
            .field("dbf", &self.dbf())
            .field("reset", &self.reset())
            .field("ttcm", &self.ttcm())
            .field("abom", &self.abom())
            .field("awum", &self.awum())
            .field("nart", &self.nart())
            .field("rflm", &self.rflm())
            .field("txfp", &self.txfp())
            .field("sleep", &self.sleep())
            .field("inrq", &self.inrq())
            .finish()
    }
}
impl W {
    ///Bit 0 - INRQ
    #[inline(always)]
    pub fn inrq(&mut self) -> INRQ_W<'_, MCRrs> {
        INRQ_W::new(self, 0)
    }
    ///Bit 1 - SLEEP
    #[inline(always)]
    pub fn sleep(&mut self) -> SLEEP_W<'_, MCRrs> {
        SLEEP_W::new(self, 1)
    }
    ///Bit 2 - TXFP
    #[inline(always)]
    pub fn txfp(&mut self) -> TXFP_W<'_, MCRrs> {
        TXFP_W::new(self, 2)
    }
    ///Bit 3 - RFLM
    #[inline(always)]
    pub fn rflm(&mut self) -> RFLM_W<'_, MCRrs> {
        RFLM_W::new(self, 3)
    }
    ///Bit 4 - NART
    #[inline(always)]
    pub fn nart(&mut self) -> NART_W<'_, MCRrs> {
        NART_W::new(self, 4)
    }
    ///Bit 5 - AWUM
    #[inline(always)]
    pub fn awum(&mut self) -> AWUM_W<'_, MCRrs> {
        AWUM_W::new(self, 5)
    }
    ///Bit 6 - ABOM
    #[inline(always)]
    pub fn abom(&mut self) -> ABOM_W<'_, MCRrs> {
        ABOM_W::new(self, 6)
    }
    ///Bit 7 - TTCM
    #[inline(always)]
    pub fn ttcm(&mut self) -> TTCM_W<'_, MCRrs> {
        TTCM_W::new(self, 7)
    }
    ///Bit 15 - RESET
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W<'_, MCRrs> {
        RESET_W::new(self, 15)
    }
    ///Bit 16 - DBF
    #[inline(always)]
    pub fn dbf(&mut self) -> DBF_W<'_, MCRrs> {
        DBF_W::new(self, 16)
    }
}
/**CAN_MCR

You can [`read`](crate::Reg::read) this register and get [`mcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F103.html#CAN:MCR)*/
pub struct MCRrs;
impl crate::RegisterSpec for MCRrs {
    type Ux = u32;
}
///`read()` method returns [`mcr::R`](R) reader structure
impl crate::Readable for MCRrs {}
///`write(|w| ..)` method takes [`mcr::W`](W) writer structure
impl crate::Writable for MCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MCR to value 0x0001_0002
impl crate::Resettable for MCRrs {
    const RESET_VALUE: u32 = 0x0001_0002;
}
