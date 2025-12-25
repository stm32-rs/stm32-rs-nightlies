///Register `IC20CFGR` reader
pub type R = crate::R<IC20CFGRrs>;
///Register `IC20CFGR` writer
pub type W = crate::W<IC20CFGRrs>;
///Field `IC20INT` reader - Divider IC20 integer division factor
pub type IC20INT_R = crate::FieldReader;
///Field `IC20INT` writer - Divider IC20 integer division factor
pub type IC20INT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `IC20SEL` reader - Divider IC20 Source Selection
pub type IC20SEL_R = crate::FieldReader;
///Field `IC20SEL` writer - Divider IC20 Source Selection
pub type IC20SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 16:23 - Divider IC20 integer division factor
    #[inline(always)]
    pub fn ic20int(&self) -> IC20INT_R {
        IC20INT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 28:29 - Divider IC20 Source Selection
    #[inline(always)]
    pub fn ic20sel(&self) -> IC20SEL_R {
        IC20SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IC20CFGR")
            .field("ic20int", &self.ic20int())
            .field("ic20sel", &self.ic20sel())
            .finish()
    }
}
impl W {
    ///Bits 16:23 - Divider IC20 integer division factor
    #[inline(always)]
    pub fn ic20int(&mut self) -> IC20INT_W<'_, IC20CFGRrs> {
        IC20INT_W::new(self, 16)
    }
    ///Bits 28:29 - Divider IC20 Source Selection
    #[inline(always)]
    pub fn ic20sel(&mut self) -> IC20SEL_W<'_, IC20CFGRrs> {
        IC20SEL_W::new(self, 28)
    }
}
/**RCC IC20 configuration register

You can [`read`](crate::Reg::read) this register and get [`ic20cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic20cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:IC20CFGR)*/
pub struct IC20CFGRrs;
impl crate::RegisterSpec for IC20CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`ic20cfgr::R`](R) reader structure
impl crate::Readable for IC20CFGRrs {}
///`write(|w| ..)` method takes [`ic20cfgr::W`](W) writer structure
impl crate::Writable for IC20CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IC20CFGR to value 0x3000_0000
impl crate::Resettable for IC20CFGRrs {
    const RESET_VALUE: u32 = 0x3000_0000;
}
