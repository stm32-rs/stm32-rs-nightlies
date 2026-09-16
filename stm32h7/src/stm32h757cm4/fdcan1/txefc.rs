///Register `TXEFC` reader
pub type R = crate::R<TXEFCrs>;
///Register `TXEFC` writer
pub type W = crate::W<TXEFCrs>;
///Field `EFSA` reader - Event FIFO Start Address
pub type EFSA_R = crate::FieldReader<u16>;
///Field `EFSA` writer - Event FIFO Start Address
pub type EFSA_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
///Field `EFS` reader - Event FIFO Size
pub type EFS_R = crate::FieldReader;
///Field `EFS` writer - Event FIFO Size
pub type EFS_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `EFWM` reader - Event FIFO Watermark
pub type EFWM_R = crate::FieldReader;
///Field `EFWM` writer - Event FIFO Watermark
pub type EFWM_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 2:15 - Event FIFO Start Address
    #[inline(always)]
    pub fn efsa(&self) -> EFSA_R {
        EFSA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    ///Bits 16:21 - Event FIFO Size
    #[inline(always)]
    pub fn efs(&self) -> EFS_R {
        EFS_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    ///Bits 24:29 - Event FIFO Watermark
    #[inline(always)]
    pub fn efwm(&self) -> EFWM_R {
        EFWM_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXEFC")
            .field("efsa", &self.efsa())
            .field("efs", &self.efs())
            .field("efwm", &self.efwm())
            .finish()
    }
}
impl W {
    ///Bits 2:15 - Event FIFO Start Address
    #[inline(always)]
    pub fn efsa(&mut self) -> EFSA_W<'_, TXEFCrs> {
        EFSA_W::new(self, 2)
    }
    ///Bits 16:21 - Event FIFO Size
    #[inline(always)]
    pub fn efs(&mut self) -> EFS_W<'_, TXEFCrs> {
        EFS_W::new(self, 16)
    }
    ///Bits 24:29 - Event FIFO Watermark
    #[inline(always)]
    pub fn efwm(&mut self) -> EFWM_W<'_, TXEFCrs> {
        EFWM_W::new(self, 24)
    }
}
/**FDCAN Tx Event FIFO Configuration Register

You can [`read`](crate::Reg::read) this register and get [`txefc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txefc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#FDCAN1:TXEFC)*/
pub struct TXEFCrs;
impl crate::RegisterSpec for TXEFCrs {
    type Ux = u32;
}
///`read()` method returns [`txefc::R`](R) reader structure
impl crate::Readable for TXEFCrs {}
///`write(|w| ..)` method takes [`txefc::W`](W) writer structure
impl crate::Writable for TXEFCrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TXEFC to value 0
impl crate::Resettable for TXEFCrs {}
