///Register `SCRATCHR%s` reader
pub type R = crate::R<SCRATCHRrs>;
///Register `SCRATCHR%s` writer
pub type W = crate::W<SCRATCHRrs>;
///Field `SDATA` reader - Scratch data
pub type SDATA_R = crate::FieldReader<u32>;
///Field `SDATA` writer - Scratch data
pub type SDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Scratch data
    #[inline(always)]
    pub fn sdata(&self) -> SDATA_R {
        SDATA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCRATCHR")
            .field("sdata", &self.sdata())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Scratch data
    #[inline(always)]
    pub fn sdata(&mut self) -> SDATA_W<'_, SCRATCHRrs> {
        SDATA_W::new(self, 0)
    }
}
/**BSEC scratch register %s

You can [`read`](crate::Reg::read) this register and get [`scratchr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scratchr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#BSEC:SCRATCHR[0])*/
pub struct SCRATCHRrs;
impl crate::RegisterSpec for SCRATCHRrs {
    type Ux = u32;
}
///`read()` method returns [`scratchr::R`](R) reader structure
impl crate::Readable for SCRATCHRrs {}
///`write(|w| ..)` method takes [`scratchr::W`](W) writer structure
impl crate::Writable for SCRATCHRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SCRATCHR%s to value 0
impl crate::Resettable for SCRATCHRrs {}
