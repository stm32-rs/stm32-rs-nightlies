///Register `IC5CFGR` reader
pub type R = crate::R<IC5CFGRrs>;
///Register `IC5CFGR` writer
pub type W = crate::W<IC5CFGRrs>;
///Field `IC5INT` reader - Divider IC5 integer division factor
pub type IC5INT_R = crate::FieldReader;
///Field `IC5INT` writer - Divider IC5 integer division factor
pub type IC5INT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `IC5SEL` reader - Divider IC5 Source Selection
pub type IC5SEL_R = crate::FieldReader;
///Field `IC5SEL` writer - Divider IC5 Source Selection
pub type IC5SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 16:23 - Divider IC5 integer division factor
    #[inline(always)]
    pub fn ic5int(&self) -> IC5INT_R {
        IC5INT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 28:29 - Divider IC5 Source Selection
    #[inline(always)]
    pub fn ic5sel(&self) -> IC5SEL_R {
        IC5SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IC5CFGR")
            .field("ic5int", &self.ic5int())
            .field("ic5sel", &self.ic5sel())
            .finish()
    }
}
impl W {
    ///Bits 16:23 - Divider IC5 integer division factor
    #[inline(always)]
    pub fn ic5int(&mut self) -> IC5INT_W<'_, IC5CFGRrs> {
        IC5INT_W::new(self, 16)
    }
    ///Bits 28:29 - Divider IC5 Source Selection
    #[inline(always)]
    pub fn ic5sel(&mut self) -> IC5SEL_W<'_, IC5CFGRrs> {
        IC5SEL_W::new(self, 28)
    }
}
/**RCC IC5 configuration register

You can [`read`](crate::Reg::read) this register and get [`ic5cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic5cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:IC5CFGR)*/
pub struct IC5CFGRrs;
impl crate::RegisterSpec for IC5CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`ic5cfgr::R`](R) reader structure
impl crate::Readable for IC5CFGRrs {}
///`write(|w| ..)` method takes [`ic5cfgr::W`](W) writer structure
impl crate::Writable for IC5CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IC5CFGR to value 0
impl crate::Resettable for IC5CFGRrs {}
