///Register `IC15CFGR` reader
pub type R = crate::R<IC15CFGRrs>;
///Register `IC15CFGR` writer
pub type W = crate::W<IC15CFGRrs>;
///Field `IC15INT` reader - Divider IC15 integer division factor
pub type IC15INT_R = crate::FieldReader;
///Field `IC15INT` writer - Divider IC15 integer division factor
pub type IC15INT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `IC15SEL` reader - Divider IC15 Source Selection
pub type IC15SEL_R = crate::FieldReader;
///Field `IC15SEL` writer - Divider IC15 Source Selection
pub type IC15SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 16:23 - Divider IC15 integer division factor
    #[inline(always)]
    pub fn ic15int(&self) -> IC15INT_R {
        IC15INT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 28:29 - Divider IC15 Source Selection
    #[inline(always)]
    pub fn ic15sel(&self) -> IC15SEL_R {
        IC15SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IC15CFGR")
            .field("ic15int", &self.ic15int())
            .field("ic15sel", &self.ic15sel())
            .finish()
    }
}
impl W {
    ///Bits 16:23 - Divider IC15 integer division factor
    #[inline(always)]
    pub fn ic15int(&mut self) -> IC15INT_W<'_, IC15CFGRrs> {
        IC15INT_W::new(self, 16)
    }
    ///Bits 28:29 - Divider IC15 Source Selection
    #[inline(always)]
    pub fn ic15sel(&mut self) -> IC15SEL_W<'_, IC15CFGRrs> {
        IC15SEL_W::new(self, 28)
    }
}
/**RCC IC15 configuration register

You can [`read`](crate::Reg::read) this register and get [`ic15cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic15cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:IC15CFGR)*/
pub struct IC15CFGRrs;
impl crate::RegisterSpec for IC15CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`ic15cfgr::R`](R) reader structure
impl crate::Readable for IC15CFGRrs {}
///`write(|w| ..)` method takes [`ic15cfgr::W`](W) writer structure
impl crate::Writable for IC15CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IC15CFGR to value 0x2000_0000
impl crate::Resettable for IC15CFGRrs {
    const RESET_VALUE: u32 = 0x2000_0000;
}
