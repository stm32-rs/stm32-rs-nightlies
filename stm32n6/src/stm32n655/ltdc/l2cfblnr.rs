///Register `L2CFBLNR` reader
pub type R = crate::R<L2CFBLNRrs>;
///Register `L2CFBLNR` writer
pub type W = crate::W<L2CFBLNRrs>;
///Field `CFBLNBR` reader - frame buffer line number
pub type CFBLNBR_R = crate::FieldReader<u16>;
///Field `CFBLNBR` writer - frame buffer line number
pub type CFBLNBR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - frame buffer line number
    #[inline(always)]
    pub fn cfblnbr(&self) -> CFBLNBR_R {
        CFBLNBR_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2CFBLNR")
            .field("cfblnbr", &self.cfblnbr())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - frame buffer line number
    #[inline(always)]
    pub fn cfblnbr(&mut self) -> CFBLNBR_W<'_, L2CFBLNRrs> {
        CFBLNBR_W::new(self, 0)
    }
}
/**LTDC layerx color frame buffer line number register

You can [`read`](crate::Reg::read) this register and get [`l2cfblnr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2cfblnr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#LTDC:L2CFBLNR)*/
pub struct L2CFBLNRrs;
impl crate::RegisterSpec for L2CFBLNRrs {
    type Ux = u32;
}
///`read()` method returns [`l2cfblnr::R`](R) reader structure
impl crate::Readable for L2CFBLNRrs {}
///`write(|w| ..)` method takes [`l2cfblnr::W`](W) writer structure
impl crate::Writable for L2CFBLNRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets L2CFBLNR to value 0
impl crate::Resettable for L2CFBLNRrs {}
