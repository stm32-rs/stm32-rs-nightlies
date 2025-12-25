///Register `AESLEPRIVCMDREG` reader
pub type R = crate::R<AESLEPRIVCMDREGrs>;
///Register `AESLEPRIVCMDREG` writer
pub type W = crate::W<AESLEPRIVCMDREGrs>;
///Field `START` writer - AES Le privacy Start command.
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INTENA` reader - AES Le privacy interrupt enable on Interrupt2Reg
pub type INTENA_R = crate::BitReader;
///Field `INTENA` writer - AES Le privacy interrupt enable on Interrupt2Reg
pub type INTENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NBKEYS` reader - AES Le privacy number of keys pointed by AesLePrivPointerReg (points to the resolution key list.
pub type NBKEYS_R = crate::FieldReader;
///Field `NBKEYS` writer - AES Le privacy number of keys pointed by AesLePrivPointerReg (points to the resolution key list.
pub type NBKEYS_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bit 1 - AES Le privacy interrupt enable on Interrupt2Reg
    #[inline(always)]
    pub fn intena(&self) -> INTENA_R {
        INTENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:9 - AES Le privacy number of keys pointed by AesLePrivPointerReg (points to the resolution key list.
    #[inline(always)]
    pub fn nbkeys(&self) -> NBKEYS_R {
        NBKEYS_R::new(((self.bits >> 2) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AESLEPRIVCMDREG")
            .field("intena", &self.intena())
            .field("nbkeys", &self.nbkeys())
            .finish()
    }
}
impl W {
    ///Bit 0 - AES Le privacy Start command.
    #[inline(always)]
    pub fn start(&mut self) -> START_W<'_, AESLEPRIVCMDREGrs> {
        START_W::new(self, 0)
    }
    ///Bit 1 - AES Le privacy interrupt enable on Interrupt2Reg
    #[inline(always)]
    pub fn intena(&mut self) -> INTENA_W<'_, AESLEPRIVCMDREGrs> {
        INTENA_W::new(self, 1)
    }
    ///Bits 2:9 - AES Le privacy number of keys pointed by AesLePrivPointerReg (points to the resolution key list.
    #[inline(always)]
    pub fn nbkeys(&mut self) -> NBKEYS_W<'_, AESLEPRIVCMDREGrs> {
        NBKEYS_W::new(self, 2)
    }
}
/**AESLEPRIVCMDREG register

You can [`read`](crate::Reg::read) this register and get [`aesleprivcmdreg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesleprivcmdreg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#BLUE:AESLEPRIVCMDREG)*/
pub struct AESLEPRIVCMDREGrs;
impl crate::RegisterSpec for AESLEPRIVCMDREGrs {
    type Ux = u32;
}
///`read()` method returns [`aesleprivcmdreg::R`](R) reader structure
impl crate::Readable for AESLEPRIVCMDREGrs {}
///`write(|w| ..)` method takes [`aesleprivcmdreg::W`](W) writer structure
impl crate::Writable for AESLEPRIVCMDREGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AESLEPRIVCMDREG to value 0
impl crate::Resettable for AESLEPRIVCMDREGrs {}
