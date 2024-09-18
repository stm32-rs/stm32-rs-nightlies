///Register `BMCMPR6` reader
pub type R = crate::R<BMCMPR6rs>;
///Register `BMCMPR6` writer
pub type W = crate::W<BMCMPR6rs>;
///Field `BMCMP` reader - BMCMP
pub type BMCMP_R = crate::FieldReader<u16>;
///Field `BMCMP` writer - BMCMP
pub type BMCMP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - BMCMP
    #[inline(always)]
    pub fn bmcmp(&self) -> BMCMP_R {
        BMCMP_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BMCMPR6")
            .field("bmcmp", &self.bmcmp())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - BMCMP
    #[inline(always)]
    #[must_use]
    pub fn bmcmp(&mut self) -> BMCMP_W<BMCMPR6rs> {
        BMCMP_W::new(self, 0)
    }
}
/**BMCMPR6

You can [`read`](crate::Reg::read) this register and get [`bmcmpr6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bmcmpr6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#HRTIM_Common:BMCMPR6)*/
pub struct BMCMPR6rs;
impl crate::RegisterSpec for BMCMPR6rs {
    type Ux = u32;
}
///`read()` method returns [`bmcmpr6::R`](R) reader structure
impl crate::Readable for BMCMPR6rs {}
///`write(|w| ..)` method takes [`bmcmpr6::W`](W) writer structure
impl crate::Writable for BMCMPR6rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BMCMPR6 to value 0
impl crate::Resettable for BMCMPR6rs {
    const RESET_VALUE: u32 = 0;
}
