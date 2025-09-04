///Register `IORETENRA` reader
pub type R = crate::R<IORETENRArs>;
///Register `IORETENRA` writer
pub type W = crate::W<IORETENRArs>;
///Field `EN0` reader - Port A Standby GPIO retention enable Access can be protected by GPIOA SECy, privilege protection is controlled by PWR SPRIV or PWR NSPRIV. When set, each bit enables the Standby GPIO retention feature for PAy
pub type EN0_R = crate::BitReader;
///Field `EN0` writer - Port A Standby GPIO retention enable Access can be protected by GPIOA SECy, privilege protection is controlled by PWR SPRIV or PWR NSPRIV. When set, each bit enables the Standby GPIO retention feature for PAy
pub type EN0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EN1` reader - Port A Standby GPIO retention enable Access can be protected by GPIOA SECy, privilege protection is controlled by PWR SPRIV or PWR NSPRIV. When set, each bit enables the Standby GPIO retention feature for PAy
pub type EN1_R = crate::BitReader;
///Field `EN1` writer - Port A Standby GPIO retention enable Access can be protected by GPIOA SECy, privilege protection is controlled by PWR SPRIV or PWR NSPRIV. When set, each bit enables the Standby GPIO retention feature for PAy
pub type EN1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EN2` reader - Port A Standby GPIO retention enable Access can be protected by GPIOA SECy, privilege protection is controlled by PWR SPRIV or PWR NSPRIV. When set, each bit enables the Standby GPIO retention feature for PAy
pub type EN2_R = crate::BitReader;
///Field `EN2` writer - Port A Standby GPIO retention enable Access can be protected by GPIOA SECy, privilege protection is controlled by PWR SPRIV or PWR NSPRIV. When set, each bit enables the Standby GPIO retention feature for PAy
pub type EN2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EN3` reader - Port A Standby GPIO retention enable Access can be protected by GPIOA SECy, privilege protection is controlled by PWR SPRIV or PWR NSPRIV. When set, each bit enables the Standby GPIO retention feature for PAy
pub type EN3_R = crate::BitReader;
///Field `EN3` writer - Port A Standby GPIO retention enable Access can be protected by GPIOA SECy, privilege protection is controlled by PWR SPRIV or PWR NSPRIV. When set, each bit enables the Standby GPIO retention feature for PAy
pub type EN3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EN5` reader - Port A Standby GPIO retention enable Access can be secured by GPIOA SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type EN5_R = crate::BitReader;
///Field `EN5` writer - Port A Standby GPIO retention enable Access can be secured by GPIOA SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type EN5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EN6` reader - Port A Standby GPIO retention enable Access can be secured by GPIOA SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type EN6_R = crate::BitReader;
///Field `EN6` writer - Port A Standby GPIO retention enable Access can be secured by GPIOA SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type EN6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EN7` reader - Port A Standby GPIO retention enable Access can be secured by GPIOA SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type EN7_R = crate::BitReader;
///Field `EN7` writer - Port A Standby GPIO retention enable Access can be secured by GPIOA SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type EN7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EN8` reader - Port A Standby GPIO retention enable Access can be secured by GPIOA SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type EN8_R = crate::BitReader;
///Field `EN8` writer - Port A Standby GPIO retention enable Access can be secured by GPIOA SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type EN8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EN9` reader - Port A Standby GPIO retention enable Access can be secured by GPIOA SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type EN9_R = crate::BitReader;
///Field `EN9` writer - Port A Standby GPIO retention enable Access can be secured by GPIOA SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type EN9_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EN10` reader - Port A Standby GPIO retention enable Access can be secured by GPIOA SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type EN10_R = crate::BitReader;
///Field `EN10` writer - Port A Standby GPIO retention enable Access can be secured by GPIOA SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type EN10_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EN11` reader - Port A Standby GPIO retention enable Access can be secured by GPIOA SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type EN11_R = crate::BitReader;
///Field `EN11` writer - Port A Standby GPIO retention enable Access can be secured by GPIOA SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type EN11_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EN12` reader - Port A Standby GPIO retention enable Access can be secured by GPIOA SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type EN12_R = crate::BitReader;
///Field `EN12` writer - Port A Standby GPIO retention enable Access can be secured by GPIOA SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type EN12_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EN13` reader - Port A Standby GPIO retention enable Access can be secured by GPIOA SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type EN13_R = crate::BitReader;
///Field `EN13` writer - Port A Standby GPIO retention enable Access can be secured by GPIOA SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type EN13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EN14` reader - Port A Standby GPIO retention enable Access can be secured by GPIOA SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type EN14_R = crate::BitReader;
///Field `EN14` writer - Port A Standby GPIO retention enable Access can be secured by GPIOA SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type EN14_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EN15` reader - Port A Standby GPIO retention enable Access can be secured by GPIOA SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type EN15_R = crate::BitReader;
///Field `EN15` writer - Port A Standby GPIO retention enable Access can be secured by GPIOA SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
pub type EN15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Port A Standby GPIO retention enable Access can be protected by GPIOA SECy, privilege protection is controlled by PWR SPRIV or PWR NSPRIV. When set, each bit enables the Standby GPIO retention feature for PAy
    #[inline(always)]
    pub fn en0(&self) -> EN0_R {
        EN0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port A Standby GPIO retention enable Access can be protected by GPIOA SECy, privilege protection is controlled by PWR SPRIV or PWR NSPRIV. When set, each bit enables the Standby GPIO retention feature for PAy
    #[inline(always)]
    pub fn en1(&self) -> EN1_R {
        EN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Port A Standby GPIO retention enable Access can be protected by GPIOA SECy, privilege protection is controlled by PWR SPRIV or PWR NSPRIV. When set, each bit enables the Standby GPIO retention feature for PAy
    #[inline(always)]
    pub fn en2(&self) -> EN2_R {
        EN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Port A Standby GPIO retention enable Access can be protected by GPIOA SECy, privilege protection is controlled by PWR SPRIV or PWR NSPRIV. When set, each bit enables the Standby GPIO retention feature for PAy
    #[inline(always)]
    pub fn en3(&self) -> EN3_R {
        EN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - Port A Standby GPIO retention enable Access can be secured by GPIOA SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn en5(&self) -> EN5_R {
        EN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Port A Standby GPIO retention enable Access can be secured by GPIOA SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn en6(&self) -> EN6_R {
        EN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Port A Standby GPIO retention enable Access can be secured by GPIOA SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn en7(&self) -> EN7_R {
        EN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Port A Standby GPIO retention enable Access can be secured by GPIOA SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn en8(&self) -> EN8_R {
        EN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Port A Standby GPIO retention enable Access can be secured by GPIOA SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn en9(&self) -> EN9_R {
        EN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Port A Standby GPIO retention enable Access can be secured by GPIOA SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn en10(&self) -> EN10_R {
        EN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Port A Standby GPIO retention enable Access can be secured by GPIOA SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn en11(&self) -> EN11_R {
        EN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Port A Standby GPIO retention enable Access can be secured by GPIOA SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn en12(&self) -> EN12_R {
        EN12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Port A Standby GPIO retention enable Access can be secured by GPIOA SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn en13(&self) -> EN13_R {
        EN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Port A Standby GPIO retention enable Access can be secured by GPIOA SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn en14(&self) -> EN14_R {
        EN14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Port A Standby GPIO retention enable Access can be secured by GPIOA SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn en15(&self) -> EN15_R {
        EN15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IORETENRA")
            .field("en0", &self.en0())
            .field("en1", &self.en1())
            .field("en2", &self.en2())
            .field("en3", &self.en3())
            .field("en5", &self.en5())
            .field("en6", &self.en6())
            .field("en7", &self.en7())
            .field("en8", &self.en8())
            .field("en9", &self.en9())
            .field("en10", &self.en10())
            .field("en11", &self.en11())
            .field("en12", &self.en12())
            .field("en13", &self.en13())
            .field("en14", &self.en14())
            .field("en15", &self.en15())
            .finish()
    }
}
impl W {
    ///Bit 0 - Port A Standby GPIO retention enable Access can be protected by GPIOA SECy, privilege protection is controlled by PWR SPRIV or PWR NSPRIV. When set, each bit enables the Standby GPIO retention feature for PAy
    #[inline(always)]
    pub fn en0(&mut self) -> EN0_W<IORETENRArs> {
        EN0_W::new(self, 0)
    }
    ///Bit 1 - Port A Standby GPIO retention enable Access can be protected by GPIOA SECy, privilege protection is controlled by PWR SPRIV or PWR NSPRIV. When set, each bit enables the Standby GPIO retention feature for PAy
    #[inline(always)]
    pub fn en1(&mut self) -> EN1_W<IORETENRArs> {
        EN1_W::new(self, 1)
    }
    ///Bit 2 - Port A Standby GPIO retention enable Access can be protected by GPIOA SECy, privilege protection is controlled by PWR SPRIV or PWR NSPRIV. When set, each bit enables the Standby GPIO retention feature for PAy
    #[inline(always)]
    pub fn en2(&mut self) -> EN2_W<IORETENRArs> {
        EN2_W::new(self, 2)
    }
    ///Bit 3 - Port A Standby GPIO retention enable Access can be protected by GPIOA SECy, privilege protection is controlled by PWR SPRIV or PWR NSPRIV. When set, each bit enables the Standby GPIO retention feature for PAy
    #[inline(always)]
    pub fn en3(&mut self) -> EN3_W<IORETENRArs> {
        EN3_W::new(self, 3)
    }
    ///Bit 5 - Port A Standby GPIO retention enable Access can be secured by GPIOA SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn en5(&mut self) -> EN5_W<IORETENRArs> {
        EN5_W::new(self, 5)
    }
    ///Bit 6 - Port A Standby GPIO retention enable Access can be secured by GPIOA SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn en6(&mut self) -> EN6_W<IORETENRArs> {
        EN6_W::new(self, 6)
    }
    ///Bit 7 - Port A Standby GPIO retention enable Access can be secured by GPIOA SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn en7(&mut self) -> EN7_W<IORETENRArs> {
        EN7_W::new(self, 7)
    }
    ///Bit 8 - Port A Standby GPIO retention enable Access can be secured by GPIOA SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn en8(&mut self) -> EN8_W<IORETENRArs> {
        EN8_W::new(self, 8)
    }
    ///Bit 9 - Port A Standby GPIO retention enable Access can be secured by GPIOA SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn en9(&mut self) -> EN9_W<IORETENRArs> {
        EN9_W::new(self, 9)
    }
    ///Bit 10 - Port A Standby GPIO retention enable Access can be secured by GPIOA SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn en10(&mut self) -> EN10_W<IORETENRArs> {
        EN10_W::new(self, 10)
    }
    ///Bit 11 - Port A Standby GPIO retention enable Access can be secured by GPIOA SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn en11(&mut self) -> EN11_W<IORETENRArs> {
        EN11_W::new(self, 11)
    }
    ///Bit 12 - Port A Standby GPIO retention enable Access can be secured by GPIOA SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn en12(&mut self) -> EN12_W<IORETENRArs> {
        EN12_W::new(self, 12)
    }
    ///Bit 13 - Port A Standby GPIO retention enable Access can be secured by GPIOA SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn en13(&mut self) -> EN13_W<IORETENRArs> {
        EN13_W::new(self, 13)
    }
    ///Bit 14 - Port A Standby GPIO retention enable Access can be secured by GPIOA SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn en14(&mut self) -> EN14_W<IORETENRArs> {
        EN14_W::new(self, 14)
    }
    ///Bit 15 - Port A Standby GPIO retention enable Access can be secured by GPIOA SECy. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with PWR SPRIV or when non-secure with PWR NSPRIV.
    #[inline(always)]
    pub fn en15(&mut self) -> EN15_W<IORETENRArs> {
        EN15_W::new(self, 15)
    }
}
/**PWR port A Standby IO retention enable register

You can [`read`](crate::Reg::read) this register and get [`ioretenra::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ioretenra::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#PWR:IORETENRA)*/
pub struct IORETENRArs;
impl crate::RegisterSpec for IORETENRArs {
    type Ux = u32;
}
///`read()` method returns [`ioretenra::R`](R) reader structure
impl crate::Readable for IORETENRArs {}
///`write(|w| ..)` method takes [`ioretenra::W`](W) writer structure
impl crate::Writable for IORETENRArs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IORETENRA to value 0
impl crate::Resettable for IORETENRArs {}
