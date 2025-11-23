///Register `IC3CFGR` reader
pub type R = crate::R<IC3CFGRrs>;
///Register `IC3CFGR` writer
pub type W = crate::W<IC3CFGRrs>;
///Field `IC3INT` reader - Divider IC3 integer division factor
pub type IC3INT_R = crate::FieldReader;
///Field `IC3INT` writer - Divider IC3 integer division factor
pub type IC3INT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `IC3SEL` reader - Divider IC3 Source Selection
pub type IC3SEL_R = crate::FieldReader;
///Field `IC3SEL` writer - Divider IC3 Source Selection
pub type IC3SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 16:23 - Divider IC3 integer division factor
    #[inline(always)]
    pub fn ic3int(&self) -> IC3INT_R {
        IC3INT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 28:29 - Divider IC3 Source Selection
    #[inline(always)]
    pub fn ic3sel(&self) -> IC3SEL_R {
        IC3SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IC3CFGR")
            .field("ic3int", &self.ic3int())
            .field("ic3sel", &self.ic3sel())
            .finish()
    }
}
impl W {
    ///Bits 16:23 - Divider IC3 integer division factor
    #[inline(always)]
    pub fn ic3int(&mut self) -> IC3INT_W<'_, IC3CFGRrs> {
        IC3INT_W::new(self, 16)
    }
    ///Bits 28:29 - Divider IC3 Source Selection
    #[inline(always)]
    pub fn ic3sel(&mut self) -> IC3SEL_W<'_, IC3CFGRrs> {
        IC3SEL_W::new(self, 28)
    }
}
/**RCC IC3 configuration register

You can [`read`](crate::Reg::read) this register and get [`ic3cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic3cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:IC3CFGR)*/
pub struct IC3CFGRrs;
impl crate::RegisterSpec for IC3CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`ic3cfgr::R`](R) reader structure
impl crate::Readable for IC3CFGRrs {}
///`write(|w| ..)` method takes [`ic3cfgr::W`](W) writer structure
impl crate::Writable for IC3CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IC3CFGR to value 0
impl crate::Resettable for IC3CFGRrs {}
