#[doc = "Register `GPIOD_LCKR` reader"]
pub type R = crate::R<GPIOD_LCKRrs>;
#[doc = "Register `GPIOD_LCKR` writer"]
pub type W = crate::W<GPIOD_LCKRrs>;
#[doc = "Field `LCK0` reader - LCK0"]
pub type LCK0_R = crate::BitReader;
#[doc = "Field `LCK0` writer - LCK0"]
pub type LCK0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCK1` reader - LCK1"]
pub type LCK1_R = crate::BitReader;
#[doc = "Field `LCK1` writer - LCK1"]
pub type LCK1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCK2` reader - LCK2"]
pub type LCK2_R = crate::BitReader;
#[doc = "Field `LCK2` writer - LCK2"]
pub type LCK2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCK3` reader - LCK3"]
pub type LCK3_R = crate::BitReader;
#[doc = "Field `LCK3` writer - LCK3"]
pub type LCK3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCK4` reader - LCK4"]
pub type LCK4_R = crate::BitReader;
#[doc = "Field `LCK4` writer - LCK4"]
pub type LCK4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCK5` reader - LCK5"]
pub type LCK5_R = crate::BitReader;
#[doc = "Field `LCK5` writer - LCK5"]
pub type LCK5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCK6` reader - LCK6"]
pub type LCK6_R = crate::BitReader;
#[doc = "Field `LCK6` writer - LCK6"]
pub type LCK6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCK7` reader - LCK7"]
pub type LCK7_R = crate::BitReader;
#[doc = "Field `LCK7` writer - LCK7"]
pub type LCK7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCK8` reader - LCK8"]
pub type LCK8_R = crate::BitReader;
#[doc = "Field `LCK8` writer - LCK8"]
pub type LCK8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCK9` reader - LCK9"]
pub type LCK9_R = crate::BitReader;
#[doc = "Field `LCK9` writer - LCK9"]
pub type LCK9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCK10` reader - LCK10"]
pub type LCK10_R = crate::BitReader;
#[doc = "Field `LCK10` writer - LCK10"]
pub type LCK10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCK11` reader - LCK11"]
pub type LCK11_R = crate::BitReader;
#[doc = "Field `LCK11` writer - LCK11"]
pub type LCK11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCK12` reader - LCK12"]
pub type LCK12_R = crate::BitReader;
#[doc = "Field `LCK12` writer - LCK12"]
pub type LCK12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCK13` reader - LCK13"]
pub type LCK13_R = crate::BitReader;
#[doc = "Field `LCK13` writer - LCK13"]
pub type LCK13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCK14` reader - LCK14"]
pub type LCK14_R = crate::BitReader;
#[doc = "Field `LCK14` writer - LCK14"]
pub type LCK14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCK15` reader - LCK15"]
pub type LCK15_R = crate::BitReader;
#[doc = "Field `LCK15` writer - LCK15"]
pub type LCK15_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCKK` reader - LCKK"]
pub type LCKK_R = crate::BitReader;
#[doc = "Field `LCKK` writer - LCKK"]
pub type LCKK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LCK0"]
    #[inline(always)]
    pub fn lck0(&self) -> LCK0_R {
        LCK0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LCK1"]
    #[inline(always)]
    pub fn lck1(&self) -> LCK1_R {
        LCK1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LCK2"]
    #[inline(always)]
    pub fn lck2(&self) -> LCK2_R {
        LCK2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LCK3"]
    #[inline(always)]
    pub fn lck3(&self) -> LCK3_R {
        LCK3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LCK4"]
    #[inline(always)]
    pub fn lck4(&self) -> LCK4_R {
        LCK4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LCK5"]
    #[inline(always)]
    pub fn lck5(&self) -> LCK5_R {
        LCK5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LCK6"]
    #[inline(always)]
    pub fn lck6(&self) -> LCK6_R {
        LCK6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LCK7"]
    #[inline(always)]
    pub fn lck7(&self) -> LCK7_R {
        LCK7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LCK8"]
    #[inline(always)]
    pub fn lck8(&self) -> LCK8_R {
        LCK8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LCK9"]
    #[inline(always)]
    pub fn lck9(&self) -> LCK9_R {
        LCK9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LCK10"]
    #[inline(always)]
    pub fn lck10(&self) -> LCK10_R {
        LCK10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LCK11"]
    #[inline(always)]
    pub fn lck11(&self) -> LCK11_R {
        LCK11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LCK12"]
    #[inline(always)]
    pub fn lck12(&self) -> LCK12_R {
        LCK12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - LCK13"]
    #[inline(always)]
    pub fn lck13(&self) -> LCK13_R {
        LCK13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - LCK14"]
    #[inline(always)]
    pub fn lck14(&self) -> LCK14_R {
        LCK14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - LCK15"]
    #[inline(always)]
    pub fn lck15(&self) -> LCK15_R {
        LCK15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - LCKK"]
    #[inline(always)]
    pub fn lckk(&self) -> LCKK_R {
        LCKK_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCK0"]
    #[inline(always)]
    #[must_use]
    pub fn lck0(&mut self) -> LCK0_W<GPIOD_LCKRrs> {
        LCK0_W::new(self, 0)
    }
    #[doc = "Bit 1 - LCK1"]
    #[inline(always)]
    #[must_use]
    pub fn lck1(&mut self) -> LCK1_W<GPIOD_LCKRrs> {
        LCK1_W::new(self, 1)
    }
    #[doc = "Bit 2 - LCK2"]
    #[inline(always)]
    #[must_use]
    pub fn lck2(&mut self) -> LCK2_W<GPIOD_LCKRrs> {
        LCK2_W::new(self, 2)
    }
    #[doc = "Bit 3 - LCK3"]
    #[inline(always)]
    #[must_use]
    pub fn lck3(&mut self) -> LCK3_W<GPIOD_LCKRrs> {
        LCK3_W::new(self, 3)
    }
    #[doc = "Bit 4 - LCK4"]
    #[inline(always)]
    #[must_use]
    pub fn lck4(&mut self) -> LCK4_W<GPIOD_LCKRrs> {
        LCK4_W::new(self, 4)
    }
    #[doc = "Bit 5 - LCK5"]
    #[inline(always)]
    #[must_use]
    pub fn lck5(&mut self) -> LCK5_W<GPIOD_LCKRrs> {
        LCK5_W::new(self, 5)
    }
    #[doc = "Bit 6 - LCK6"]
    #[inline(always)]
    #[must_use]
    pub fn lck6(&mut self) -> LCK6_W<GPIOD_LCKRrs> {
        LCK6_W::new(self, 6)
    }
    #[doc = "Bit 7 - LCK7"]
    #[inline(always)]
    #[must_use]
    pub fn lck7(&mut self) -> LCK7_W<GPIOD_LCKRrs> {
        LCK7_W::new(self, 7)
    }
    #[doc = "Bit 8 - LCK8"]
    #[inline(always)]
    #[must_use]
    pub fn lck8(&mut self) -> LCK8_W<GPIOD_LCKRrs> {
        LCK8_W::new(self, 8)
    }
    #[doc = "Bit 9 - LCK9"]
    #[inline(always)]
    #[must_use]
    pub fn lck9(&mut self) -> LCK9_W<GPIOD_LCKRrs> {
        LCK9_W::new(self, 9)
    }
    #[doc = "Bit 10 - LCK10"]
    #[inline(always)]
    #[must_use]
    pub fn lck10(&mut self) -> LCK10_W<GPIOD_LCKRrs> {
        LCK10_W::new(self, 10)
    }
    #[doc = "Bit 11 - LCK11"]
    #[inline(always)]
    #[must_use]
    pub fn lck11(&mut self) -> LCK11_W<GPIOD_LCKRrs> {
        LCK11_W::new(self, 11)
    }
    #[doc = "Bit 12 - LCK12"]
    #[inline(always)]
    #[must_use]
    pub fn lck12(&mut self) -> LCK12_W<GPIOD_LCKRrs> {
        LCK12_W::new(self, 12)
    }
    #[doc = "Bit 13 - LCK13"]
    #[inline(always)]
    #[must_use]
    pub fn lck13(&mut self) -> LCK13_W<GPIOD_LCKRrs> {
        LCK13_W::new(self, 13)
    }
    #[doc = "Bit 14 - LCK14"]
    #[inline(always)]
    #[must_use]
    pub fn lck14(&mut self) -> LCK14_W<GPIOD_LCKRrs> {
        LCK14_W::new(self, 14)
    }
    #[doc = "Bit 15 - LCK15"]
    #[inline(always)]
    #[must_use]
    pub fn lck15(&mut self) -> LCK15_W<GPIOD_LCKRrs> {
        LCK15_W::new(self, 15)
    }
    #[doc = "Bit 16 - LCKK"]
    #[inline(always)]
    #[must_use]
    pub fn lckk(&mut self) -> LCKK_W<GPIOD_LCKRrs> {
        LCKK_W::new(self, 16)
    }
}
#[doc = "This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiod_lckr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiod_lckr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIOD_LCKRrs;
impl crate::RegisterSpec for GPIOD_LCKRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpiod_lckr::R`](R) reader structure"]
impl crate::Readable for GPIOD_LCKRrs {}
#[doc = "`write(|w| ..)` method takes [`gpiod_lckr::W`](W) writer structure"]
impl crate::Writable for GPIOD_LCKRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIOD_LCKR to value 0"]
impl crate::Resettable for GPIOD_LCKRrs {
    const RESET_VALUE: u32 = 0;
}
