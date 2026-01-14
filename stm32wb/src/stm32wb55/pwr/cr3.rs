///Register `CR3` reader
pub type R = crate::R<CR3rs>;
///Register `CR3` writer
pub type W = crate::W<CR3rs>;
///Field `EWUP1` reader - Enable Wakeup pin WKUP1
pub type EWUP1_R = crate::BitReader;
///Field `EWUP1` writer - Enable Wakeup pin WKUP1
pub type EWUP1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWUP2` reader - Enable Wakeup pin WKUP2
pub type EWUP2_R = crate::BitReader;
///Field `EWUP2` writer - Enable Wakeup pin WKUP2
pub type EWUP2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWUP3` reader - Enable Wakeup pin WKUP3
pub type EWUP3_R = crate::BitReader;
///Field `EWUP3` writer - Enable Wakeup pin WKUP3
pub type EWUP3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWUP4` reader - Enable Wakeup pin WKUP4
pub type EWUP4_R = crate::BitReader;
///Field `EWUP4` writer - Enable Wakeup pin WKUP4
pub type EWUP4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWUP5` reader - Enable Wakeup pin WKUP5
pub type EWUP5_R = crate::BitReader;
///Field `EWUP5` writer - Enable Wakeup pin WKUP5
pub type EWUP5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EBORHSDFB` reader - Enable BORH and Step Down counverter forced in Bypass interrups for CPU1
pub type EBORHSDFB_R = crate::BitReader;
///Field `EBORHSDFB` writer - Enable BORH and Step Down counverter forced in Bypass interrups for CPU1
pub type EBORHSDFB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RRS` reader - SRAM2a retention in Standby mode
pub type RRS_R = crate::BitReader;
///Field `RRS` writer - SRAM2a retention in Standby mode
pub type RRS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APC` reader - Apply pull-up and pull-down configuration
pub type APC_R = crate::BitReader;
///Field `APC` writer - Apply pull-up and pull-down configuration
pub type APC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EBLEA` reader - Enable BLE end of activity interrupt for CPU1
pub type EBLEA_R = crate::BitReader;
///Field `EBLEA` writer - Enable BLE end of activity interrupt for CPU1
pub type EBLEA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ECRPE` reader - Enable critical radio phase end of activity interrupt for CPU1
pub type ECRPE_R = crate::BitReader;
///Field `ECRPE` writer - Enable critical radio phase end of activity interrupt for CPU1
pub type ECRPE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `E802A` reader - Enable end of activity interrupt for CPU1
pub type E802A_R = crate::BitReader;
///Field `E802A` writer - Enable end of activity interrupt for CPU1
pub type E802A_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EC2H` reader - Enable CPU2 Hold interrupt for CPU1
pub type EC2H_R = crate::BitReader;
///Field `EC2H` writer - Enable CPU2 Hold interrupt for CPU1
pub type EC2H_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EIWUL` reader - Enable internal wakeup line for CPU1
pub type EIWUL_R = crate::BitReader;
///Field `EIWUL` writer - Enable internal wakeup line for CPU1
pub type EIWUL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Enable Wakeup pin WKUP1
    #[inline(always)]
    pub fn ewup1(&self) -> EWUP1_R {
        EWUP1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Enable Wakeup pin WKUP2
    #[inline(always)]
    pub fn ewup2(&self) -> EWUP2_R {
        EWUP2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Enable Wakeup pin WKUP3
    #[inline(always)]
    pub fn ewup3(&self) -> EWUP3_R {
        EWUP3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Enable Wakeup pin WKUP4
    #[inline(always)]
    pub fn ewup4(&self) -> EWUP4_R {
        EWUP4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Enable Wakeup pin WKUP5
    #[inline(always)]
    pub fn ewup5(&self) -> EWUP5_R {
        EWUP5_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - Enable BORH and Step Down counverter forced in Bypass interrups for CPU1
    #[inline(always)]
    pub fn eborhsdfb(&self) -> EBORHSDFB_R {
        EBORHSDFB_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SRAM2a retention in Standby mode
    #[inline(always)]
    pub fn rrs(&self) -> RRS_R {
        RRS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Apply pull-up and pull-down configuration
    #[inline(always)]
    pub fn apc(&self) -> APC_R {
        APC_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Enable BLE end of activity interrupt for CPU1
    #[inline(always)]
    pub fn eblea(&self) -> EBLEA_R {
        EBLEA_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Enable critical radio phase end of activity interrupt for CPU1
    #[inline(always)]
    pub fn ecrpe(&self) -> ECRPE_R {
        ECRPE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Enable end of activity interrupt for CPU1
    #[inline(always)]
    pub fn e802a(&self) -> E802A_R {
        E802A_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Enable CPU2 Hold interrupt for CPU1
    #[inline(always)]
    pub fn ec2h(&self) -> EC2H_R {
        EC2H_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Enable internal wakeup line for CPU1
    #[inline(always)]
    pub fn eiwul(&self) -> EIWUL_R {
        EIWUL_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR3")
            .field("eiwul", &self.eiwul())
            .field("ec2h", &self.ec2h())
            .field("e802a", &self.e802a())
            .field("eblea", &self.eblea())
            .field("ecrpe", &self.ecrpe())
            .field("apc", &self.apc())
            .field("rrs", &self.rrs())
            .field("eborhsdfb", &self.eborhsdfb())
            .field("ewup5", &self.ewup5())
            .field("ewup4", &self.ewup4())
            .field("ewup3", &self.ewup3())
            .field("ewup2", &self.ewup2())
            .field("ewup1", &self.ewup1())
            .finish()
    }
}
impl W {
    ///Bit 0 - Enable Wakeup pin WKUP1
    #[inline(always)]
    pub fn ewup1(&mut self) -> EWUP1_W<'_, CR3rs> {
        EWUP1_W::new(self, 0)
    }
    ///Bit 1 - Enable Wakeup pin WKUP2
    #[inline(always)]
    pub fn ewup2(&mut self) -> EWUP2_W<'_, CR3rs> {
        EWUP2_W::new(self, 1)
    }
    ///Bit 2 - Enable Wakeup pin WKUP3
    #[inline(always)]
    pub fn ewup3(&mut self) -> EWUP3_W<'_, CR3rs> {
        EWUP3_W::new(self, 2)
    }
    ///Bit 3 - Enable Wakeup pin WKUP4
    #[inline(always)]
    pub fn ewup4(&mut self) -> EWUP4_W<'_, CR3rs> {
        EWUP4_W::new(self, 3)
    }
    ///Bit 4 - Enable Wakeup pin WKUP5
    #[inline(always)]
    pub fn ewup5(&mut self) -> EWUP5_W<'_, CR3rs> {
        EWUP5_W::new(self, 4)
    }
    ///Bit 8 - Enable BORH and Step Down counverter forced in Bypass interrups for CPU1
    #[inline(always)]
    pub fn eborhsdfb(&mut self) -> EBORHSDFB_W<'_, CR3rs> {
        EBORHSDFB_W::new(self, 8)
    }
    ///Bit 9 - SRAM2a retention in Standby mode
    #[inline(always)]
    pub fn rrs(&mut self) -> RRS_W<'_, CR3rs> {
        RRS_W::new(self, 9)
    }
    ///Bit 10 - Apply pull-up and pull-down configuration
    #[inline(always)]
    pub fn apc(&mut self) -> APC_W<'_, CR3rs> {
        APC_W::new(self, 10)
    }
    ///Bit 11 - Enable BLE end of activity interrupt for CPU1
    #[inline(always)]
    pub fn eblea(&mut self) -> EBLEA_W<'_, CR3rs> {
        EBLEA_W::new(self, 11)
    }
    ///Bit 12 - Enable critical radio phase end of activity interrupt for CPU1
    #[inline(always)]
    pub fn ecrpe(&mut self) -> ECRPE_W<'_, CR3rs> {
        ECRPE_W::new(self, 12)
    }
    ///Bit 13 - Enable end of activity interrupt for CPU1
    #[inline(always)]
    pub fn e802a(&mut self) -> E802A_W<'_, CR3rs> {
        E802A_W::new(self, 13)
    }
    ///Bit 14 - Enable CPU2 Hold interrupt for CPU1
    #[inline(always)]
    pub fn ec2h(&mut self) -> EC2H_W<'_, CR3rs> {
        EC2H_W::new(self, 14)
    }
    ///Bit 15 - Enable internal wakeup line for CPU1
    #[inline(always)]
    pub fn eiwul(&mut self) -> EIWUL_W<'_, CR3rs> {
        EIWUL_W::new(self, 15)
    }
}
/**Power control register 3

You can [`read`](crate::Reg::read) this register and get [`cr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#PWR:CR3)*/
pub struct CR3rs;
impl crate::RegisterSpec for CR3rs {
    type Ux = u32;
}
///`read()` method returns [`cr3::R`](R) reader structure
impl crate::Readable for CR3rs {}
///`write(|w| ..)` method takes [`cr3::W`](W) writer structure
impl crate::Writable for CR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR3 to value 0x8000
impl crate::Resettable for CR3rs {
    const RESET_VALUE: u32 = 0x8000;
}
