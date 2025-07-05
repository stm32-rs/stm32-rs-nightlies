///Register `ODR` reader
pub type R = crate::R<ODRrs>;
///Register `ODR` writer
pub type W = crate::W<ODRrs>;
///Field `OD3` reader - Port H output data I/O pin 3 This bits can be read and written by software. Access can be protected by GPIOH SEC3. Note: For atomic bit set/reset, the OD bit can be individually set and/or reset by writing to the GPIOH_BSRR or GPIOH_BRR registers.
pub type OD3_R = crate::BitReader;
///Field `OD3` writer - Port H output data I/O pin 3 This bits can be read and written by software. Access can be protected by GPIOH SEC3. Note: For atomic bit set/reset, the OD bit can be individually set and/or reset by writing to the GPIOH_BSRR or GPIOH_BRR registers.
pub type OD3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 3 - Port H output data I/O pin 3 This bits can be read and written by software. Access can be protected by GPIOH SEC3. Note: For atomic bit set/reset, the OD bit can be individually set and/or reset by writing to the GPIOH_BSRR or GPIOH_BRR registers.
    #[inline(always)]
    pub fn od3(&self) -> OD3_R {
        OD3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ODR").field("od3", &self.od3()).finish()
    }
}
impl W {
    ///Bit 3 - Port H output data I/O pin 3 This bits can be read and written by software. Access can be protected by GPIOH SEC3. Note: For atomic bit set/reset, the OD bit can be individually set and/or reset by writing to the GPIOH_BSRR or GPIOH_BRR registers.
    #[inline(always)]
    pub fn od3(&mut self) -> OD3_W<ODRrs> {
        OD3_W::new(self, 3)
    }
}
/**GPIO port H output data register

You can [`read`](crate::Reg::read) this register and get [`odr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`odr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#GPIOH:ODR)*/
pub struct ODRrs;
impl crate::RegisterSpec for ODRrs {
    type Ux = u32;
}
///`read()` method returns [`odr::R`](R) reader structure
impl crate::Readable for ODRrs {}
///`write(|w| ..)` method takes [`odr::W`](W) writer structure
impl crate::Writable for ODRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ODR to value 0
impl crate::Resettable for ODRrs {}
