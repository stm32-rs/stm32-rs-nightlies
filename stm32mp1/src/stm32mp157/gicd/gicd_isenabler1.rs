///Register `GICD_ISENABLER1` reader
pub type R = crate::R<GICD_ISENABLER1rs>;
///Register `GICD_ISENABLER1` writer
pub type W = crate::W<GICD_ISENABLER1rs>;
///Field `ISENABLER1` reader - ISENABLER1
pub type ISENABLER1_R = crate::FieldReader<u32>;
///Field `ISENABLER1` writer - ISENABLER1
pub type ISENABLER1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ISENABLER1
    #[inline(always)]
    pub fn isenabler1(&self) -> ISENABLER1_R {
        ISENABLER1_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ISENABLER1")
            .field("isenabler1", &self.isenabler1())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ISENABLER1
    #[inline(always)]
    #[must_use]
    pub fn isenabler1(&mut self) -> ISENABLER1_W<GICD_ISENABLER1rs> {
        ISENABLER1_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`gicd_isenabler1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_isenabler1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:GICD_ISENABLER1)*/
pub struct GICD_ISENABLER1rs;
impl crate::RegisterSpec for GICD_ISENABLER1rs {
    type Ux = u32;
}
///`read()` method returns [`gicd_isenabler1::R`](R) reader structure
impl crate::Readable for GICD_ISENABLER1rs {}
///`write(|w| ..)` method takes [`gicd_isenabler1::W`](W) writer structure
impl crate::Writable for GICD_ISENABLER1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GICD_ISENABLER1 to value 0
impl crate::Resettable for GICD_ISENABLER1rs {
    const RESET_VALUE: u32 = 0;
}
