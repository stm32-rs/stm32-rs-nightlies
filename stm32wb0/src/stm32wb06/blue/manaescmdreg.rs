///Register `MANAESCMDREG` reader
pub type R = crate::R<MANAESCMDREGrs>;
///Register `MANAESCMDREG` writer
pub type W = crate::W<MANAESCMDREGrs>;
///Field `START` writer - AES Manual encryption Start command.
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INTENA` reader - AES Manual encryption interrupt enable on Interrupt2Reg
pub type INTENA_R = crate::BitReader;
///Field `INTENA` writer - AES Manual encryption interrupt enable on Interrupt2Reg
pub type INTENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - AES Manual encryption interrupt enable on Interrupt2Reg
    #[inline(always)]
    pub fn intena(&self) -> INTENA_R {
        INTENA_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MANAESCMDREG")
            .field("intena", &self.intena())
            .finish()
    }
}
impl W {
    ///Bit 0 - AES Manual encryption Start command.
    #[inline(always)]
    pub fn start(&mut self) -> START_W<'_, MANAESCMDREGrs> {
        START_W::new(self, 0)
    }
    ///Bit 1 - AES Manual encryption interrupt enable on Interrupt2Reg
    #[inline(always)]
    pub fn intena(&mut self) -> INTENA_W<'_, MANAESCMDREGrs> {
        INTENA_W::new(self, 1)
    }
}
/**MANAESCMDREG register

You can [`read`](crate::Reg::read) this register and get [`manaescmdreg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`manaescmdreg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#BLUE:MANAESCMDREG)*/
pub struct MANAESCMDREGrs;
impl crate::RegisterSpec for MANAESCMDREGrs {
    type Ux = u32;
}
///`read()` method returns [`manaescmdreg::R`](R) reader structure
impl crate::Readable for MANAESCMDREGrs {}
///`write(|w| ..)` method takes [`manaescmdreg::W`](W) writer structure
impl crate::Writable for MANAESCMDREGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MANAESCMDREG to value 0
impl crate::Resettable for MANAESCMDREGrs {}
