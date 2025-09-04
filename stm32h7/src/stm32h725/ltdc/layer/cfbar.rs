///Register `CFBAR` reader
pub type R = crate::R<CFBARrs>;
///Register `CFBAR` writer
pub type W = crate::W<CFBARrs>;
///Field `CFBADD` reader - Color Frame Buffer Start Address
pub type CFBADD_R = crate::FieldReader<u32>;
///Field `CFBADD` writer - Color Frame Buffer Start Address
pub type CFBADD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl R {
    ///Bits 0:31 - Color Frame Buffer Start Address
    #[inline(always)]
    pub fn cfbadd(&self) -> CFBADD_R {
        CFBADD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFBAR")
            .field("cfbadd", &self.cfbadd())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Color Frame Buffer Start Address
    #[inline(always)]
    pub fn cfbadd(&mut self) -> CFBADD_W<CFBARrs> {
        CFBADD_W::new(self, 0)
    }
}
/**Layerx Color Frame Buffer Address Register

You can [`read`](crate::Reg::read) this register and get [`cfbar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfbar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CFBARrs;
impl crate::RegisterSpec for CFBARrs {
    type Ux = u32;
}
///`read()` method returns [`cfbar::R`](R) reader structure
impl crate::Readable for CFBARrs {}
///`write(|w| ..)` method takes [`cfbar::W`](W) writer structure
impl crate::Writable for CFBARrs {
    type Safety = crate::Safe;
}
///`reset()` method sets CFBAR to value 0
impl crate::Resettable for CFBARrs {}
