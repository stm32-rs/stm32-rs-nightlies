///Register `PMC` reader
pub type R = crate::R<PMCrs>;
///Register `PMC` writer
pub type W = crate::W<PMCrs>;
/**Ethernet PHY interface selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MII_RMII_SEL {
    ///0: MII interface is selected
    Mii = 0,
    ///1: RMII PHY interface is selected
    RmiiPhy = 1,
}
impl From<MII_RMII_SEL> for bool {
    #[inline(always)]
    fn from(variant: MII_RMII_SEL) -> Self {
        variant as u8 != 0
    }
}
///Field `MII_RMII_SEL` reader - Ethernet PHY interface selection
pub type MII_RMII_SEL_R = crate::BitReader<MII_RMII_SEL>;
impl MII_RMII_SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MII_RMII_SEL {
        match self.bits {
            false => MII_RMII_SEL::Mii,
            true => MII_RMII_SEL::RmiiPhy,
        }
    }
    ///MII interface is selected
    #[inline(always)]
    pub fn is_mii(&self) -> bool {
        *self == MII_RMII_SEL::Mii
    }
    ///RMII PHY interface is selected
    #[inline(always)]
    pub fn is_rmii_phy(&self) -> bool {
        *self == MII_RMII_SEL::RmiiPhy
    }
}
///Field `MII_RMII_SEL` writer - Ethernet PHY interface selection
pub type MII_RMII_SEL_W<'a, REG> = crate::BitWriter<'a, REG, MII_RMII_SEL>;
impl<'a, REG> MII_RMII_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///MII interface is selected
    #[inline(always)]
    pub fn mii(self) -> &'a mut crate::W<REG> {
        self.variant(MII_RMII_SEL::Mii)
    }
    ///RMII PHY interface is selected
    #[inline(always)]
    pub fn rmii_phy(self) -> &'a mut crate::W<REG> {
        self.variant(MII_RMII_SEL::RmiiPhy)
    }
}
impl R {
    ///Bit 23 - Ethernet PHY interface selection
    #[inline(always)]
    pub fn mii_rmii_sel(&self) -> MII_RMII_SEL_R {
        MII_RMII_SEL_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMC")
            .field("mii_rmii_sel", &self.mii_rmii_sel())
            .finish()
    }
}
impl W {
    ///Bit 23 - Ethernet PHY interface selection
    #[inline(always)]
    pub fn mii_rmii_sel(&mut self) -> MII_RMII_SEL_W<PMCrs> {
        MII_RMII_SEL_W::new(self, 23)
    }
}
/**peripheral mode configuration register

You can [`read`](crate::Reg::read) this register and get [`pmc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F407.html#SYSCFG:PMC)*/
pub struct PMCrs;
impl crate::RegisterSpec for PMCrs {
    type Ux = u32;
}
///`read()` method returns [`pmc::R`](R) reader structure
impl crate::Readable for PMCrs {}
///`write(|w| ..)` method takes [`pmc::W`](W) writer structure
impl crate::Writable for PMCrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PMC to value 0
impl crate::Resettable for PMCrs {}
