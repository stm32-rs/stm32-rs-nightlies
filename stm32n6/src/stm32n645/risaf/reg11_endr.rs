///Register `REG11_ENDR` reader
pub type R = crate::R<REG11_ENDRrs>;
///Register `REG11_ENDR` writer
pub type W = crate::W<REG11_ENDRrs>;
///Field `BADDEND` reader - Base region address end
pub type BADDEND_R = crate::FieldReader<u32>;
///Field `BADDEND` writer - Base region address end
pub type BADDEND_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Base region address end
    #[inline(always)]
    pub fn baddend(&self) -> BADDEND_R {
        BADDEND_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REG11_ENDR")
            .field("baddend", &self.baddend())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Base region address end
    #[inline(always)]
    pub fn baddend(&mut self) -> BADDEND_W<REG11_ENDRrs> {
        BADDEND_W::new(self, 0)
    }
}
/**RISAF region 11 end-address register

You can [`read`](crate::Reg::read) this register and get [`reg11_endr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg11_endr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RISAF:REG11_ENDR)*/
pub struct REG11_ENDRrs;
impl crate::RegisterSpec for REG11_ENDRrs {
    type Ux = u32;
}
///`read()` method returns [`reg11_endr::R`](R) reader structure
impl crate::Readable for REG11_ENDRrs {}
///`write(|w| ..)` method takes [`reg11_endr::W`](W) writer structure
impl crate::Writable for REG11_ENDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets REG11_ENDR to value 0x0fff
impl crate::Resettable for REG11_ENDRrs {
    const RESET_VALUE: u32 = 0x0fff;
}
