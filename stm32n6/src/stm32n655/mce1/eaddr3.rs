///Register `EADDR3` reader
pub type R = crate::R<EADDR3rs>;
///Register `EADDR3` writer
pub type W = crate::W<EADDR3rs>;
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
        f.debug_struct("EADDR3")
            .field("baddend", &self.baddend())
            .finish()
    }
}
impl W {
    ///Bits 12:31 - Region address end
    #[inline(always)]
    pub fn baddend(&mut self) -> BADDEND_W<'_, EADDR3rs> {
        BADDEND_W::new(self, 12)
    }
}
/**MCE end address for region 3 register

You can [`read`](crate::Reg::read) this register and get [`eaddr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eaddr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:EADDR3)*/
pub struct EADDR3rs;
impl crate::RegisterSpec for EADDR3rs {
    type Ux = u32;
}
///`read()` method returns [`eaddr3::R`](R) reader structure
impl crate::Readable for EADDR3rs {}
///`write(|w| ..)` method takes [`eaddr3::W`](W) writer structure
impl crate::Writable for EADDR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EADDR3 to value 0x0fff
impl crate::Resettable for EADDR3rs {
    const RESET_VALUE: u32 = 0x0fff;
}
