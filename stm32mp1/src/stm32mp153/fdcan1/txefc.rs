///Register `TXEFC` reader
pub type R = crate::R<TXEFCrs>;
///Register `TXEFC` writer
pub type W = crate::W<TXEFCrs>;
///Field `EFSA` reader - EFSA
pub type EFSA_R = crate::FieldReader<u16>;
///Field `EFSA` writer - EFSA
pub type EFSA_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
///Field `EFS` reader - EFS
pub type EFS_R = crate::FieldReader;
///Field `EFS` writer - EFS
pub type EFS_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `EFWM` reader - EFWM
pub type EFWM_R = crate::FieldReader;
///Field `EFWM` writer - EFWM
pub type EFWM_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 2:15 - EFSA
    #[inline(always)]
    pub fn efsa(&self) -> EFSA_R {
        EFSA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    ///Bits 16:21 - EFS
    #[inline(always)]
    pub fn efs(&self) -> EFS_R {
        EFS_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    ///Bits 24:29 - EFWM
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
    ///Bits 2:15 - EFSA
    #[inline(always)]
    pub fn efsa(&mut self) -> EFSA_W<TXEFCrs> {
        EFSA_W::new(self, 2)
    }
    ///Bits 16:21 - EFS
    #[inline(always)]
    pub fn efs(&mut self) -> EFS_W<TXEFCrs> {
        EFS_W::new(self, 16)
    }
    ///Bits 24:29 - EFWM
    #[inline(always)]
    pub fn efwm(&mut self) -> EFWM_W<TXEFCrs> {
        EFWM_W::new(self, 24)
    }
}
/**FDCAN Tx event FIFO configuration register

You can [`read`](crate::Reg::read) this register and get [`txefc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txefc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:TXEFC)*/
pub struct TXEFCrs;
impl crate::RegisterSpec for TXEFCrs {
    type Ux = u32;
}
///`read()` method returns [`txefc::R`](R) reader structure
impl crate::Readable for TXEFCrs {}
///`write(|w| ..)` method takes [`txefc::W`](W) writer structure
impl crate::Writable for TXEFCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TXEFC to value 0
impl crate::Resettable for TXEFCrs {
    const RESET_VALUE: u32 = 0;
}