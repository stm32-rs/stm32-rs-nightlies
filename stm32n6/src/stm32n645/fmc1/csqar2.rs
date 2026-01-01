///Register `CSQAR2` reader
pub type R = crate::R<CSQAR2rs>;
///Register `CSQAR2` writer
pub type W = crate::W<CSQAR2rs>;
///Field `ADDC5` reader - Address Cycle 5
pub type ADDC5_R = crate::FieldReader;
///Field `ADDC5` writer - Address Cycle 5
pub type ADDC5_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `SAO` reader - Spare Area Address Offset
pub type SAO_R = crate::FieldReader<u16>;
///Field `SAO` writer - Spare Area Address Offset
pub type SAO_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:7 - Address Cycle 5
    #[inline(always)]
    pub fn addc5(&self) -> ADDC5_R {
        ADDC5_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:31 - Spare Area Address Offset
    #[inline(always)]
    pub fn sao(&self) -> SAO_R {
        SAO_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSQAR2")
            .field("addc5", &self.addc5())
            .field("sao", &self.sao())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Address Cycle 5
    #[inline(always)]
    pub fn addc5(&mut self) -> ADDC5_W<'_, CSQAR2rs> {
        ADDC5_W::new(self, 0)
    }
    ///Bits 16:31 - Spare Area Address Offset
    #[inline(always)]
    pub fn sao(&mut self) -> SAO_W<'_, CSQAR2rs> {
        SAO_W::new(self, 16)
    }
}
/**FMC NAND command sequencer address register 2

You can [`read`](crate::Reg::read) this register and get [`csqar2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csqar2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#FMC1:CSQAR2)*/
pub struct CSQAR2rs;
impl crate::RegisterSpec for CSQAR2rs {
    type Ux = u32;
}
///`read()` method returns [`csqar2::R`](R) reader structure
impl crate::Readable for CSQAR2rs {}
///`write(|w| ..)` method takes [`csqar2::W`](W) writer structure
impl crate::Writable for CSQAR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSQAR2 to value 0
impl crate::Resettable for CSQAR2rs {}
