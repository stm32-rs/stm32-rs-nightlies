///Register `IC1CFGR` reader
pub type R = crate::R<IC1CFGRrs>;
///Register `IC1CFGR` writer
pub type W = crate::W<IC1CFGRrs>;
///Field `IC1INT` reader - Divider IC1 integer division factor
pub type IC1INT_R = crate::FieldReader;
///Field `IC1INT` writer - Divider IC1 integer division factor
pub type IC1INT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `IC1SEL` reader - Divider IC1 Source Selection
pub type IC1SEL_R = crate::FieldReader;
///Field `IC1SEL` writer - Divider IC1 Source Selection
pub type IC1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 16:23 - Divider IC1 integer division factor
    #[inline(always)]
    pub fn ic1int(&self) -> IC1INT_R {
        IC1INT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 28:29 - Divider IC1 Source Selection
    #[inline(always)]
    pub fn ic1sel(&self) -> IC1SEL_R {
        IC1SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IC1CFGR")
            .field("ic1int", &self.ic1int())
            .field("ic1sel", &self.ic1sel())
            .finish()
    }
}
impl W {
    ///Bits 16:23 - Divider IC1 integer division factor
    #[inline(always)]
    pub fn ic1int(&mut self) -> IC1INT_W<'_, IC1CFGRrs> {
        IC1INT_W::new(self, 16)
    }
    ///Bits 28:29 - Divider IC1 Source Selection
    #[inline(always)]
    pub fn ic1sel(&mut self) -> IC1SEL_W<'_, IC1CFGRrs> {
        IC1SEL_W::new(self, 28)
    }
}
/**RCC IC1 configuration register

You can [`read`](crate::Reg::read) this register and get [`ic1cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic1cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:IC1CFGR)*/
pub struct IC1CFGRrs;
impl crate::RegisterSpec for IC1CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`ic1cfgr::R`](R) reader structure
impl crate::Readable for IC1CFGRrs {}
///`write(|w| ..)` method takes [`ic1cfgr::W`](W) writer structure
impl crate::Writable for IC1CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IC1CFGR to value 0x0002_0000
impl crate::Resettable for IC1CFGRrs {
    const RESET_VALUE: u32 = 0x0002_0000;
}
