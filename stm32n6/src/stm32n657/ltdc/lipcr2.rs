///Register `LIPCR2` reader
pub type R = crate::R<LIPCR2rs>;
///Register `LIPCR2` writer
pub type W = crate::W<LIPCR2rs>;
///Field `LIPOS` reader - line interrupt position
pub type LIPOS_R = crate::FieldReader<u16>;
///Field `LIPOS` writer - line interrupt position
pub type LIPOS_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - line interrupt position
    #[inline(always)]
    pub fn lipos(&self) -> LIPOS_R {
        LIPOS_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LIPCR2")
            .field("lipos", &self.lipos())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - line interrupt position
    #[inline(always)]
    pub fn lipos(&mut self) -> LIPOS_W<'_, LIPCR2rs> {
        LIPOS_W::new(self, 0)
    }
}
/**LTDC line interrupt position configuration register 2

You can [`read`](crate::Reg::read) this register and get [`lipcr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lipcr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#LTDC:LIPCR2)*/
pub struct LIPCR2rs;
impl crate::RegisterSpec for LIPCR2rs {
    type Ux = u32;
}
///`read()` method returns [`lipcr2::R`](R) reader structure
impl crate::Readable for LIPCR2rs {}
///`write(|w| ..)` method takes [`lipcr2::W`](W) writer structure
impl crate::Writable for LIPCR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LIPCR2 to value 0
impl crate::Resettable for LIPCR2rs {}
