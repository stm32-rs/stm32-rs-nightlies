///Register `REG6_BENDR` reader
pub type R = crate::R<REG6_BENDRrs>;
///Register `REG6_BENDR` writer
pub type W = crate::W<REG6_BENDRrs>;
///Field `SADDEND` reader - subregion address end
pub type SADDEND_R = crate::FieldReader<u32>;
///Field `SADDEND` writer - subregion address end
pub type SADDEND_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - subregion address end
    #[inline(always)]
    pub fn saddend(&self) -> SADDEND_R {
        SADDEND_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REG6_BENDR")
            .field("saddend", &self.saddend())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - subregion address end
    #[inline(always)]
    pub fn saddend(&mut self) -> SADDEND_W<'_, REG6_BENDRrs> {
        SADDEND_W::new(self, 0)
    }
}
/**RISAF region 6 subregion B end-address register

You can [`read`](crate::Reg::read) this register and get [`reg6_bendr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg6_bendr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RISAF:REG6_BENDR)*/
pub struct REG6_BENDRrs;
impl crate::RegisterSpec for REG6_BENDRrs {
    type Ux = u32;
}
///`read()` method returns [`reg6_bendr::R`](R) reader structure
impl crate::Readable for REG6_BENDRrs {}
///`write(|w| ..)` method takes [`reg6_bendr::W`](W) writer structure
impl crate::Writable for REG6_BENDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets REG6_BENDR to value 0x0fff
impl crate::Resettable for REG6_BENDRrs {
    const RESET_VALUE: u32 = 0x0fff;
}
