///Register `M4FDRL` reader
pub type R = crate::R<M4FDRLrs>;
///Register `M4FDRL` writer
pub type W = crate::W<M4FDRLrs>;
///Field `FDATAH` reader - Failing data high (64-bit memory)
pub type FDATAH_R = crate::FieldReader<u32>;
///Field `FDATAH` writer - Failing data high (64-bit memory)
pub type FDATAH_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Failing data high (64-bit memory)
    #[inline(always)]
    pub fn fdatah(&self) -> FDATAH_R {
        FDATAH_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M4FDRL")
            .field("fdatah", &self.fdatah())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Failing data high (64-bit memory)
    #[inline(always)]
    pub fn fdatah(&mut self) -> FDATAH_W<M4FDRLrs> {
        FDATAH_W::new(self, 0)
    }
}
/**RAMECC monitor x failing data low register

You can [`read`](crate::Reg::read) this register and get [`m4fdrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m4fdrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#RAMECC1:M4FDRL)*/
pub struct M4FDRLrs;
impl crate::RegisterSpec for M4FDRLrs {
    type Ux = u32;
}
///`read()` method returns [`m4fdrl::R`](R) reader structure
impl crate::Readable for M4FDRLrs {}
///`write(|w| ..)` method takes [`m4fdrl::W`](W) writer structure
impl crate::Writable for M4FDRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets M4FDRL to value 0
impl crate::Resettable for M4FDRLrs {
    const RESET_VALUE: u32 = 0;
}
