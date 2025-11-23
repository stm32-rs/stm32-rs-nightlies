///Register `DTDR0` reader
pub type R = crate::R<DTDR0rs>;
///Register `DTDR0` writer
pub type W = crate::W<DTDR0rs>;
///Field `DTBYTE0` reader - DTBYTE0
pub type DTBYTE0_R = crate::FieldReader;
///Field `DTBYTE0` writer - DTBYTE0
pub type DTBYTE0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DTBYTE1` reader - DTBYTE1
pub type DTBYTE1_R = crate::FieldReader;
///Field `DTBYTE1` writer - DTBYTE1
pub type DTBYTE1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DTBYTE2` reader - DTBYTE2
pub type DTBYTE2_R = crate::FieldReader;
///Field `DTBYTE2` writer - DTBYTE2
pub type DTBYTE2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DTBYTE3` reader - DTBYTE3
pub type DTBYTE3_R = crate::FieldReader;
///Field `DTBYTE3` writer - DTBYTE3
pub type DTBYTE3_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - DTBYTE0
    #[inline(always)]
    pub fn dtbyte0(&self) -> DTBYTE0_R {
        DTBYTE0_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - DTBYTE1
    #[inline(always)]
    pub fn dtbyte1(&self) -> DTBYTE1_R {
        DTBYTE1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - DTBYTE2
    #[inline(always)]
    pub fn dtbyte2(&self) -> DTBYTE2_R {
        DTBYTE2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - DTBYTE3
    #[inline(always)]
    pub fn dtbyte3(&self) -> DTBYTE3_R {
        DTBYTE3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTDR0")
            .field("dtbyte0", &self.dtbyte0())
            .field("dtbyte1", &self.dtbyte1())
            .field("dtbyte2", &self.dtbyte2())
            .field("dtbyte3", &self.dtbyte3())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - DTBYTE0
    #[inline(always)]
    pub fn dtbyte0(&mut self) -> DTBYTE0_W<'_, DTDR0rs> {
        DTBYTE0_W::new(self, 0)
    }
    ///Bits 8:15 - DTBYTE1
    #[inline(always)]
    pub fn dtbyte1(&mut self) -> DTBYTE1_W<'_, DTDR0rs> {
        DTBYTE1_W::new(self, 8)
    }
    ///Bits 16:23 - DTBYTE2
    #[inline(always)]
    pub fn dtbyte2(&mut self) -> DTBYTE2_W<'_, DTDR0rs> {
        DTBYTE2_W::new(self, 16)
    }
    ///Bits 24:31 - DTBYTE3
    #[inline(always)]
    pub fn dtbyte3(&mut self) -> DTBYTE3_W<'_, DTDR0rs> {
        DTBYTE3_W::new(self, 24)
    }
}
/**DDRPHYC DTD register 0

You can [`read`](crate::Reg::read) this register and get [`dtdr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtdr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DTDR0)*/
pub struct DTDR0rs;
impl crate::RegisterSpec for DTDR0rs {
    type Ux = u32;
}
///`read()` method returns [`dtdr0::R`](R) reader structure
impl crate::Readable for DTDR0rs {}
///`write(|w| ..)` method takes [`dtdr0::W`](W) writer structure
impl crate::Writable for DTDR0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DTDR0 to value 0xdd22_ee11
impl crate::Resettable for DTDR0rs {
    const RESET_VALUE: u32 = 0xdd22_ee11;
}
