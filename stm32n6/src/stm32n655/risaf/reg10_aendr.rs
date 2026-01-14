///Register `REG10_AENDR` reader
pub type R = crate::R<REG10_AENDRrs>;
///Register `REG10_AENDR` writer
pub type W = crate::W<REG10_AENDRrs>;
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
        f.debug_struct("REG10_AENDR")
            .field("saddend", &self.saddend())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - subregion address end
    #[inline(always)]
    pub fn saddend(&mut self) -> SADDEND_W<'_, REG10_AENDRrs> {
        SADDEND_W::new(self, 0)
    }
}
/**RISAF region 10 subregion A end-address register

You can [`read`](crate::Reg::read) this register and get [`reg10_aendr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg10_aendr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RISAF:REG10_AENDR)*/
pub struct REG10_AENDRrs;
impl crate::RegisterSpec for REG10_AENDRrs {
    type Ux = u32;
}
///`read()` method returns [`reg10_aendr::R`](R) reader structure
impl crate::Readable for REG10_AENDRrs {}
///`write(|w| ..)` method takes [`reg10_aendr::W`](W) writer structure
impl crate::Writable for REG10_AENDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets REG10_AENDR to value 0x0fff
impl crate::Resettable for REG10_AENDRrs {
    const RESET_VALUE: u32 = 0x0fff;
}
