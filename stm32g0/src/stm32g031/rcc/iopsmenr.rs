#[doc = "Register `IOPSMENR` reader"]
pub type R = crate::R<IOPSMENRrs>;
#[doc = "Register `IOPSMENR` writer"]
pub type W = crate::W<IOPSMENRrs>;
#[doc = "Field `IOPASMEN` reader - I/O port A clock enable during Sleep mode"]
pub type IOPASMEN_R = crate::BitReader;
#[doc = "Field `IOPASMEN` writer - I/O port A clock enable during Sleep mode"]
pub type IOPASMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOPBSMEN` reader - I/O port B clock enable during Sleep mode"]
pub type IOPBSMEN_R = crate::BitReader;
#[doc = "Field `IOPBSMEN` writer - I/O port B clock enable during Sleep mode"]
pub type IOPBSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOPCSMEN` reader - I/O port C clock enable during Sleep mode"]
pub type IOPCSMEN_R = crate::BitReader;
#[doc = "Field `IOPCSMEN` writer - I/O port C clock enable during Sleep mode"]
pub type IOPCSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOPDSMEN` reader - I/O port D clock enable during Sleep mode"]
pub type IOPDSMEN_R = crate::BitReader;
#[doc = "Field `IOPDSMEN` writer - I/O port D clock enable during Sleep mode"]
pub type IOPDSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOPFSMEN` reader - I/O port F clock enable during Sleep mode"]
pub type IOPFSMEN_R = crate::BitReader;
#[doc = "Field `IOPFSMEN` writer - I/O port F clock enable during Sleep mode"]
pub type IOPFSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I/O port A clock enable during Sleep mode"]
    #[inline(always)]
    pub fn iopasmen(&self) -> IOPASMEN_R {
        IOPASMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I/O port B clock enable during Sleep mode"]
    #[inline(always)]
    pub fn iopbsmen(&self) -> IOPBSMEN_R {
        IOPBSMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I/O port C clock enable during Sleep mode"]
    #[inline(always)]
    pub fn iopcsmen(&self) -> IOPCSMEN_R {
        IOPCSMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I/O port D clock enable during Sleep mode"]
    #[inline(always)]
    pub fn iopdsmen(&self) -> IOPDSMEN_R {
        IOPDSMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - I/O port F clock enable during Sleep mode"]
    #[inline(always)]
    pub fn iopfsmen(&self) -> IOPFSMEN_R {
        IOPFSMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I/O port A clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn iopasmen(&mut self) -> IOPASMEN_W<IOPSMENRrs> {
        IOPASMEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - I/O port B clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn iopbsmen(&mut self) -> IOPBSMEN_W<IOPSMENRrs> {
        IOPBSMEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - I/O port C clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn iopcsmen(&mut self) -> IOPCSMEN_W<IOPSMENRrs> {
        IOPCSMEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - I/O port D clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn iopdsmen(&mut self) -> IOPDSMEN_W<IOPSMENRrs> {
        IOPDSMEN_W::new(self, 3)
    }
    #[doc = "Bit 5 - I/O port F clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn iopfsmen(&mut self) -> IOPFSMEN_W<IOPSMENRrs> {
        IOPFSMEN_W::new(self, 5)
    }
}
#[doc = "GPIO in Sleep mode clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iopsmenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iopsmenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOPSMENRrs;
impl crate::RegisterSpec for IOPSMENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iopsmenr::R`](R) reader structure"]
impl crate::Readable for IOPSMENRrs {}
#[doc = "`write(|w| ..)` method takes [`iopsmenr::W`](W) writer structure"]
impl crate::Writable for IOPSMENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOPSMENR to value 0"]
impl crate::Resettable for IOPSMENRrs {
    const RESET_VALUE: u32 = 0;
}
