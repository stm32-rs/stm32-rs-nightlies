#[doc = "Register `CR3` reader"]
pub type R = crate::R<CR3rs>;
#[doc = "Register `CR3` writer"]
pub type W = crate::W<CR3rs>;
#[doc = "Field `EWUP1` reader - Enable Wakeup pin WKUP1"]
pub type EWUP1_R = crate::BitReader;
#[doc = "Field `EWUP1` writer - Enable Wakeup pin WKUP1"]
pub type EWUP1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWUP2` reader - Enable Wakeup pin WKUP2"]
pub type EWUP2_R = crate::BitReader;
#[doc = "Field `EWUP2` writer - Enable Wakeup pin WKUP2"]
pub type EWUP2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWUP3` reader - Enable Wakeup pin WKUP3"]
pub type EWUP3_R = crate::BitReader;
#[doc = "Field `EWUP3` writer - Enable Wakeup pin WKUP3"]
pub type EWUP3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWUP4` reader - Enable Wakeup pin WKUP4"]
pub type EWUP4_R = crate::BitReader;
#[doc = "Field `EWUP4` writer - Enable Wakeup pin WKUP4"]
pub type EWUP4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWUP5` reader - Enable Wakeup pin WKUP5"]
pub type EWUP5_R = crate::BitReader;
#[doc = "Field `EWUP5` writer - Enable Wakeup pin WKUP5"]
pub type EWUP5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EBORHSDFB` reader - Enable BORH and Step Down counverter forced in Bypass interrups for CPU1"]
pub type EBORHSDFB_R = crate::BitReader;
#[doc = "Field `EBORHSDFB` writer - Enable BORH and Step Down counverter forced in Bypass interrups for CPU1"]
pub type EBORHSDFB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RRS` reader - SRAM2a retention in Standby mode"]
pub type RRS_R = crate::BitReader;
#[doc = "Field `RRS` writer - SRAM2a retention in Standby mode"]
pub type RRS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APC` reader - Apply pull-up and pull-down configuration"]
pub type APC_R = crate::BitReader;
#[doc = "Field `APC` writer - Apply pull-up and pull-down configuration"]
pub type APC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EBLEA` reader - Enable BLE end of activity interrupt for CPU1"]
pub type EBLEA_R = crate::BitReader;
#[doc = "Field `EBLEA` writer - Enable BLE end of activity interrupt for CPU1"]
pub type EBLEA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECRPE` reader - Enable critical radio phase end of activity interrupt for CPU1"]
pub type ECRPE_R = crate::BitReader;
#[doc = "Field `ECRPE` writer - Enable critical radio phase end of activity interrupt for CPU1"]
pub type ECRPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `E802A` reader - Enable end of activity interrupt for CPU1"]
pub type E802A_R = crate::BitReader;
#[doc = "Field `E802A` writer - Enable end of activity interrupt for CPU1"]
pub type E802A_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EC2H` reader - Enable CPU2 Hold interrupt for CPU1"]
pub type EC2H_R = crate::BitReader;
#[doc = "Field `EC2H` writer - Enable CPU2 Hold interrupt for CPU1"]
pub type EC2H_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIWUL` reader - Enable internal wakeup line for CPU1"]
pub type EIWUL_R = crate::BitReader;
#[doc = "Field `EIWUL` writer - Enable internal wakeup line for CPU1"]
pub type EIWUL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Wakeup pin WKUP1"]
    #[inline(always)]
    pub fn ewup1(&self) -> EWUP1_R {
        EWUP1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Wakeup pin WKUP2"]
    #[inline(always)]
    pub fn ewup2(&self) -> EWUP2_R {
        EWUP2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Wakeup pin WKUP3"]
    #[inline(always)]
    pub fn ewup3(&self) -> EWUP3_R {
        EWUP3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Wakeup pin WKUP4"]
    #[inline(always)]
    pub fn ewup4(&self) -> EWUP4_R {
        EWUP4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Wakeup pin WKUP5"]
    #[inline(always)]
    pub fn ewup5(&self) -> EWUP5_R {
        EWUP5_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable BORH and Step Down counverter forced in Bypass interrups for CPU1"]
    #[inline(always)]
    pub fn eborhsdfb(&self) -> EBORHSDFB_R {
        EBORHSDFB_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SRAM2a retention in Standby mode"]
    #[inline(always)]
    pub fn rrs(&self) -> RRS_R {
        RRS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Apply pull-up and pull-down configuration"]
    #[inline(always)]
    pub fn apc(&self) -> APC_R {
        APC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable BLE end of activity interrupt for CPU1"]
    #[inline(always)]
    pub fn eblea(&self) -> EBLEA_R {
        EBLEA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable critical radio phase end of activity interrupt for CPU1"]
    #[inline(always)]
    pub fn ecrpe(&self) -> ECRPE_R {
        ECRPE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable end of activity interrupt for CPU1"]
    #[inline(always)]
    pub fn e802a(&self) -> E802A_R {
        E802A_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable CPU2 Hold interrupt for CPU1"]
    #[inline(always)]
    pub fn ec2h(&self) -> EC2H_R {
        EC2H_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable internal wakeup line for CPU1"]
    #[inline(always)]
    pub fn eiwul(&self) -> EIWUL_R {
        EIWUL_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Wakeup pin WKUP1"]
    #[inline(always)]
    #[must_use]
    pub fn ewup1(&mut self) -> EWUP1_W<CR3rs> {
        EWUP1_W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Wakeup pin WKUP2"]
    #[inline(always)]
    #[must_use]
    pub fn ewup2(&mut self) -> EWUP2_W<CR3rs> {
        EWUP2_W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Wakeup pin WKUP3"]
    #[inline(always)]
    #[must_use]
    pub fn ewup3(&mut self) -> EWUP3_W<CR3rs> {
        EWUP3_W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Wakeup pin WKUP4"]
    #[inline(always)]
    #[must_use]
    pub fn ewup4(&mut self) -> EWUP4_W<CR3rs> {
        EWUP4_W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Wakeup pin WKUP5"]
    #[inline(always)]
    #[must_use]
    pub fn ewup5(&mut self) -> EWUP5_W<CR3rs> {
        EWUP5_W::new(self, 4)
    }
    #[doc = "Bit 8 - Enable BORH and Step Down counverter forced in Bypass interrups for CPU1"]
    #[inline(always)]
    #[must_use]
    pub fn eborhsdfb(&mut self) -> EBORHSDFB_W<CR3rs> {
        EBORHSDFB_W::new(self, 8)
    }
    #[doc = "Bit 9 - SRAM2a retention in Standby mode"]
    #[inline(always)]
    #[must_use]
    pub fn rrs(&mut self) -> RRS_W<CR3rs> {
        RRS_W::new(self, 9)
    }
    #[doc = "Bit 10 - Apply pull-up and pull-down configuration"]
    #[inline(always)]
    #[must_use]
    pub fn apc(&mut self) -> APC_W<CR3rs> {
        APC_W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable BLE end of activity interrupt for CPU1"]
    #[inline(always)]
    #[must_use]
    pub fn eblea(&mut self) -> EBLEA_W<CR3rs> {
        EBLEA_W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable critical radio phase end of activity interrupt for CPU1"]
    #[inline(always)]
    #[must_use]
    pub fn ecrpe(&mut self) -> ECRPE_W<CR3rs> {
        ECRPE_W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable end of activity interrupt for CPU1"]
    #[inline(always)]
    #[must_use]
    pub fn e802a(&mut self) -> E802A_W<CR3rs> {
        E802A_W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable CPU2 Hold interrupt for CPU1"]
    #[inline(always)]
    #[must_use]
    pub fn ec2h(&mut self) -> EC2H_W<CR3rs> {
        EC2H_W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable internal wakeup line for CPU1"]
    #[inline(always)]
    #[must_use]
    pub fn eiwul(&mut self) -> EIWUL_W<CR3rs> {
        EIWUL_W::new(self, 15)
    }
}
#[doc = "Power control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR3rs;
impl crate::RegisterSpec for CR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr3::R`](R) reader structure"]
impl crate::Readable for CR3rs {}
#[doc = "`write(|w| ..)` method takes [`cr3::W`](W) writer structure"]
impl crate::Writable for CR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR3 to value 0x8000"]
impl crate::Resettable for CR3rs {
    const RESET_VALUE: u32 = 0x8000;
}
