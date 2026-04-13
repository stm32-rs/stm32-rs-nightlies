///Register `REG5_ENDR` reader
pub type R = crate::R<REG5_ENDRrs>;
///Register `REG5_ENDR` writer
pub type W = crate::W<REG5_ENDRrs>;
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
        f.debug_struct("REG5_ENDR")
            .field("baddend", &self.baddend())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Base region address end
    #[inline(always)]
    pub fn baddend(&mut self) -> BADDEND_W<'_, REG5_ENDRrs> {
        BADDEND_W::new(self, 0)
    }
}
/**RISAF region 5 end-address register

You can [`read`](crate::Reg::read) this register and get [`reg5_endr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg5_endr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RISAF:REG5_ENDR)*/
pub struct REG5_ENDRrs;
impl crate::RegisterSpec for REG5_ENDRrs {
    type Ux = u32;
}
///`read()` method returns [`reg5_endr::R`](R) reader structure
impl crate::Readable for REG5_ENDRrs {}
///`write(|w| ..)` method takes [`reg5_endr::W`](W) writer structure
impl crate::Writable for REG5_ENDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets REG5_ENDR to value 0x0fff
impl crate::Resettable for REG5_ENDRrs {
    const RESET_VALUE: u32 = 0x0fff;
}
