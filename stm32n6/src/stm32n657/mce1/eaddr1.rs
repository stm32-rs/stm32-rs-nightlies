///Register `EADDR1` reader
pub type R = crate::R<EADDR1rs>;
///Register `EADDR1` writer
pub type W = crate::W<EADDR1rs>;
///Field `BADDEND` reader - Region address end
pub type BADDEND_R = crate::FieldReader<u32>;
///Field `BADDEND` writer - Region address end
pub type BADDEND_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    ///Bits 12:31 - Region address end
    #[inline(always)]
    pub fn baddend(&self) -> BADDEND_R {
        BADDEND_R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EADDR1")
            .field("baddend", &self.baddend())
            .finish()
    }
}
impl W {
    ///Bits 12:31 - Region address end
    #[inline(always)]
    pub fn baddend(&mut self) -> BADDEND_W<'_, EADDR1rs> {
        BADDEND_W::new(self, 12)
    }
}
/**MCE end address for region 1 register

You can [`read`](crate::Reg::read) this register and get [`eaddr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eaddr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MCE1:EADDR1)*/
pub struct EADDR1rs;
impl crate::RegisterSpec for EADDR1rs {
    type Ux = u32;
}
///`read()` method returns [`eaddr1::R`](R) reader structure
impl crate::Readable for EADDR1rs {}
///`write(|w| ..)` method takes [`eaddr1::W`](W) writer structure
impl crate::Writable for EADDR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EADDR1 to value 0x0fff
impl crate::Resettable for EADDR1rs {
    const RESET_VALUE: u32 = 0x0fff;
}
