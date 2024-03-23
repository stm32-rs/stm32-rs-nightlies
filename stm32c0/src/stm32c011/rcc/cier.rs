#[doc = "Register `CIER` reader"]
pub type R = crate::R<CIERrs>;
#[doc = "Register `CIER` writer"]
pub type W = crate::W<CIERrs>;
#[doc = "Field `LSIRDYIE` reader - LSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSI oscillator stabilization:"]
pub type LSIRDYIE_R = crate::BitReader;
#[doc = "Field `LSIRDYIE` writer - LSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSI oscillator stabilization:"]
pub type LSIRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSERDYIE` reader - LSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization:"]
pub type LSERDYIE_R = crate::BitReader;
#[doc = "Field `LSERDYIE` writer - LSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization:"]
pub type LSERDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIRDYIE` reader - HSI16 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSI16 oscillator stabilization:"]
pub type HSIRDYIE_R = crate::BitReader;
#[doc = "Field `HSIRDYIE` writer - HSI16 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSI16 oscillator stabilization:"]
pub type HSIRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSERDYIE` reader - HSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization:"]
pub type HSERDYIE_R = crate::BitReader;
#[doc = "Field `HSERDYIE` writer - HSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization:"]
pub type HSERDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSI oscillator stabilization:"]
    #[inline(always)]
    pub fn lsirdyie(&self) -> LSIRDYIE_R {
        LSIRDYIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization:"]
    #[inline(always)]
    pub fn lserdyie(&self) -> LSERDYIE_R {
        LSERDYIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - HSI16 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSI16 oscillator stabilization:"]
    #[inline(always)]
    pub fn hsirdyie(&self) -> HSIRDYIE_R {
        HSIRDYIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization:"]
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSI oscillator stabilization:"]
    #[inline(always)]
    #[must_use]
    pub fn lsirdyie(&mut self) -> LSIRDYIE_W<CIERrs> {
        LSIRDYIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization:"]
    #[inline(always)]
    #[must_use]
    pub fn lserdyie(&mut self) -> LSERDYIE_W<CIERrs> {
        LSERDYIE_W::new(self, 1)
    }
    #[doc = "Bit 3 - HSI16 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSI16 oscillator stabilization:"]
    #[inline(always)]
    #[must_use]
    pub fn hsirdyie(&mut self) -> HSIRDYIE_W<CIERrs> {
        HSIRDYIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - HSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization:"]
    #[inline(always)]
    #[must_use]
    pub fn hserdyie(&mut self) -> HSERDYIE_W<CIERrs> {
        HSERDYIE_W::new(self, 4)
    }
}
#[doc = "RCC clock interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIERrs;
impl crate::RegisterSpec for CIERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cier::R`](R) reader structure"]
impl crate::Readable for CIERrs {}
#[doc = "`write(|w| ..)` method takes [`cier::W`](W) writer structure"]
impl crate::Writable for CIERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CIER to value 0"]
impl crate::Resettable for CIERrs {
    const RESET_VALUE: u32 = 0;
}
