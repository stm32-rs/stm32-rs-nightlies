///Register `REG12_ENDR` reader
pub type R = crate::R<REG12_ENDRrs>;
///Register `REG12_ENDR` writer
pub type W = crate::W<REG12_ENDRrs>;
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
        f.debug_struct("REG12_ENDR")
            .field("baddend", &self.baddend())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Base region address end
    #[inline(always)]
    pub fn baddend(&mut self) -> BADDEND_W<'_, REG12_ENDRrs> {
        BADDEND_W::new(self, 0)
    }
}
/**RISAF region 12 end-address register

You can [`read`](crate::Reg::read) this register and get [`reg12_endr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg12_endr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG12_ENDR)*/
pub struct REG12_ENDRrs;
impl crate::RegisterSpec for REG12_ENDRrs {
    type Ux = u32;
}
///`read()` method returns [`reg12_endr::R`](R) reader structure
impl crate::Readable for REG12_ENDRrs {}
///`write(|w| ..)` method takes [`reg12_endr::W`](W) writer structure
impl crate::Writable for REG12_ENDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets REG12_ENDR to value 0x0fff
impl crate::Resettable for REG12_ENDRrs {
    const RESET_VALUE: u32 = 0x0fff;
}
