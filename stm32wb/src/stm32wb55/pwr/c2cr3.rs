///Register `C2CR3` reader
pub type R = crate::R<C2CR3rs>;
///Register `C2CR3` writer
pub type W = crate::W<C2CR3rs>;
///Field `EWUP1` reader - Enable Wakeup pin WKUP1 for CPU2
pub type EWUP1_R = crate::BitReader;
///Field `EWUP1` writer - Enable Wakeup pin WKUP1 for CPU2
pub type EWUP1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWUP2` reader - Enable Wakeup pin WKUP2 for CPU2
pub type EWUP2_R = crate::BitReader;
///Field `EWUP2` writer - Enable Wakeup pin WKUP2 for CPU2
pub type EWUP2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWUP3` reader - Enable Wakeup pin WKUP3 for CPU2
pub type EWUP3_R = crate::BitReader;
///Field `EWUP3` writer - Enable Wakeup pin WKUP3 for CPU2
pub type EWUP3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWUP4` reader - Enable Wakeup pin WKUP4 for CPU2
pub type EWUP4_R = crate::BitReader;
///Field `EWUP4` writer - Enable Wakeup pin WKUP4 for CPU2
pub type EWUP4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWUP5` reader - Enable Wakeup pin WKUP5 for CPU2
pub type EWUP5_R = crate::BitReader;
///Field `EWUP5` writer - Enable Wakeup pin WKUP5 for CPU2
pub type EWUP5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EBLEWUP` reader - Enable BLE host wakeup interrupt for CPU2
pub type EBLEWUP_R = crate::BitReader;
///Field `EBLEWUP` writer - Enable BLE host wakeup interrupt for CPU2
pub type EBLEWUP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `E802WUP` reader - Enable 802.15.4 host wakeup interrupt for CPU2
pub type E802WUP_R = crate::BitReader;
///Field `E802WUP` writer - Enable 802.15.4 host wakeup interrupt for CPU2
pub type E802WUP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APC` reader - Apply pull-up and pull-down configuration for CPU2
pub type APC_R = crate::BitReader;
///Field `APC` writer - Apply pull-up and pull-down configuration for CPU2
pub type APC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EIWUL` reader - Enable internal wakeup line for CPU2
pub type EIWUL_R = crate::BitReader;
///Field `EIWUL` writer - Enable internal wakeup line for CPU2
pub type EIWUL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Enable Wakeup pin WKUP1 for CPU2
    #[inline(always)]
    pub fn ewup1(&self) -> EWUP1_R {
        EWUP1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Enable Wakeup pin WKUP2 for CPU2
    #[inline(always)]
    pub fn ewup2(&self) -> EWUP2_R {
        EWUP2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Enable Wakeup pin WKUP3 for CPU2
    #[inline(always)]
    pub fn ewup3(&self) -> EWUP3_R {
        EWUP3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Enable Wakeup pin WKUP4 for CPU2
    #[inline(always)]
    pub fn ewup4(&self) -> EWUP4_R {
        EWUP4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Enable Wakeup pin WKUP5 for CPU2
    #[inline(always)]
    pub fn ewup5(&self) -> EWUP5_R {
        EWUP5_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 9 - Enable BLE host wakeup interrupt for CPU2
    #[inline(always)]
    pub fn eblewup(&self) -> EBLEWUP_R {
        EBLEWUP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Enable 802.15.4 host wakeup interrupt for CPU2
    #[inline(always)]
    pub fn e802wup(&self) -> E802WUP_R {
        E802WUP_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - Apply pull-up and pull-down configuration for CPU2
    #[inline(always)]
    pub fn apc(&self) -> APC_R {
        APC_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 15 - Enable internal wakeup line for CPU2
    #[inline(always)]
    pub fn eiwul(&self) -> EIWUL_R {
        EIWUL_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C2CR3")
            .field("eiwul", &self.eiwul())
            .field("apc", &self.apc())
            .field("e802wup", &self.e802wup())
            .field("eblewup", &self.eblewup())
            .field("ewup5", &self.ewup5())
            .field("ewup4", &self.ewup4())
            .field("ewup3", &self.ewup3())
            .field("ewup2", &self.ewup2())
            .field("ewup1", &self.ewup1())
            .finish()
    }
}
impl W {
    ///Bit 0 - Enable Wakeup pin WKUP1 for CPU2
    #[inline(always)]
    pub fn ewup1(&mut self) -> EWUP1_W<'_, C2CR3rs> {
        EWUP1_W::new(self, 0)
    }
    ///Bit 1 - Enable Wakeup pin WKUP2 for CPU2
    #[inline(always)]
    pub fn ewup2(&mut self) -> EWUP2_W<'_, C2CR3rs> {
        EWUP2_W::new(self, 1)
    }
    ///Bit 2 - Enable Wakeup pin WKUP3 for CPU2
    #[inline(always)]
    pub fn ewup3(&mut self) -> EWUP3_W<'_, C2CR3rs> {
        EWUP3_W::new(self, 2)
    }
    ///Bit 3 - Enable Wakeup pin WKUP4 for CPU2
    #[inline(always)]
    pub fn ewup4(&mut self) -> EWUP4_W<'_, C2CR3rs> {
        EWUP4_W::new(self, 3)
    }
    ///Bit 4 - Enable Wakeup pin WKUP5 for CPU2
    #[inline(always)]
    pub fn ewup5(&mut self) -> EWUP5_W<'_, C2CR3rs> {
        EWUP5_W::new(self, 4)
    }
    ///Bit 9 - Enable BLE host wakeup interrupt for CPU2
    #[inline(always)]
    pub fn eblewup(&mut self) -> EBLEWUP_W<'_, C2CR3rs> {
        EBLEWUP_W::new(self, 9)
    }
    ///Bit 10 - Enable 802.15.4 host wakeup interrupt for CPU2
    #[inline(always)]
    pub fn e802wup(&mut self) -> E802WUP_W<'_, C2CR3rs> {
        E802WUP_W::new(self, 10)
    }
    ///Bit 12 - Apply pull-up and pull-down configuration for CPU2
    #[inline(always)]
    pub fn apc(&mut self) -> APC_W<'_, C2CR3rs> {
        APC_W::new(self, 12)
    }
    ///Bit 15 - Enable internal wakeup line for CPU2
    #[inline(always)]
    pub fn eiwul(&mut self) -> EIWUL_W<'_, C2CR3rs> {
        EIWUL_W::new(self, 15)
    }
}
/**CPU2 Power control register 3

You can [`read`](crate::Reg::read) this register and get [`c2cr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#PWR:C2CR3)*/
pub struct C2CR3rs;
impl crate::RegisterSpec for C2CR3rs {
    type Ux = u32;
}
///`read()` method returns [`c2cr3::R`](R) reader structure
impl crate::Readable for C2CR3rs {}
///`write(|w| ..)` method takes [`c2cr3::W`](W) writer structure
impl crate::Writable for C2CR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C2CR3 to value 0x8000
impl crate::Resettable for C2CR3rs {
    const RESET_VALUE: u32 = 0x8000;
}
