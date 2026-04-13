///Register `SWIER2` reader
pub type R = crate::R<SWIER2rs>;
///Register `SWIER2` writer
pub type W = crate::W<SWIER2rs>;
///Field `SWI33` reader - Software interrupt on event
pub type SWI33_R = crate::BitReader;
///Field `SWI33` writer - Software interrupt on event
pub type SWI33_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWI40_41` reader - Software interrupt on event
pub type SWI40_41_R = crate::FieldReader;
///Field `SWI40_41` writer - Software interrupt on event
pub type SWI40_41_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 1 - Software interrupt on event
    #[inline(always)]
    pub fn swi33(&self) -> SWI33_R {
        SWI33_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 8:9 - Software interrupt on event
    #[inline(always)]
    pub fn swi40_41(&self) -> SWI40_41_R {
        SWI40_41_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWIER2")
            .field("swi33", &self.swi33())
            .field("swi40_41", &self.swi40_41())
            .finish()
    }
}
impl W {
    ///Bit 1 - Software interrupt on event
    #[inline(always)]
    pub fn swi33(&mut self) -> SWI33_W<'_, SWIER2rs> {
        SWI33_W::new(self, 1)
    }
    ///Bits 8:9 - Software interrupt on event
    #[inline(always)]
    pub fn swi40_41(&mut self) -> SWI40_41_W<'_, SWIER2rs> {
        SWI40_41_W::new(self, 8)
    }
}
/**software interrupt event register

You can [`read`](crate::Reg::read) this register and get [`swier2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#EXTI:SWIER2)*/
pub struct SWIER2rs;
impl crate::RegisterSpec for SWIER2rs {
    type Ux = u32;
}
///`read()` method returns [`swier2::R`](R) reader structure
impl crate::Readable for SWIER2rs {}
///`write(|w| ..)` method takes [`swier2::W`](W) writer structure
impl crate::Writable for SWIER2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWIER2 to value 0
impl crate::Resettable for SWIER2rs {}
