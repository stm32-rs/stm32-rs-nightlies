#[doc = "Register `CFGR2` reader"]
pub type R = crate::R<CFGR2rs>;
#[doc = "Register `CFGR2` writer"]
pub type W = crate::W<CFGR2rs>;
#[doc = "Field `IN1SEL` reader - LPTIM input 1 selection The IN1SEL bits control the LPTIM input 1 multiplexer, which connects LPTIM input 1 to one of the available inputs. For connection details refer to ."]
pub type IN1SEL_R = crate::FieldReader;
#[doc = "Field `IN1SEL` writer - LPTIM input 1 selection The IN1SEL bits control the LPTIM input 1 multiplexer, which connects LPTIM input 1 to one of the available inputs. For connection details refer to ."]
pub type IN1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IN2SEL` reader - LPTIM input 2 selection The IN2SEL bits control the LPTIM input 2 multiplexer, which connects LPTIM input 2 to one of the available inputs. For connection details refer to ."]
pub type IN2SEL_R = crate::FieldReader;
#[doc = "Field `IN2SEL` writer - LPTIM input 2 selection The IN2SEL bits control the LPTIM input 2 multiplexer, which connects LPTIM input 2 to one of the available inputs. For connection details refer to ."]
pub type IN2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC1SEL` reader - LPTIM input capture 1 selection The IC1SEL bits control the LPTIM Input capture 1 multiplexer, which connects LPTIM Input capture 1 to one of the available inputs. For connection details refer to ."]
pub type IC1SEL_R = crate::FieldReader;
#[doc = "Field `IC1SEL` writer - LPTIM input capture 1 selection The IC1SEL bits control the LPTIM Input capture 1 multiplexer, which connects LPTIM Input capture 1 to one of the available inputs. For connection details refer to ."]
pub type IC1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC2SEL` reader - LPTIM input capture 2 selection The IC2SEL bits control the LPTIM Input capture 2 multiplexer, which connects LPTIM Input capture 2 to one of the available inputs. For connection details refer to ."]
pub type IC2SEL_R = crate::FieldReader;
#[doc = "Field `IC2SEL` writer - LPTIM input capture 2 selection The IC2SEL bits control the LPTIM Input capture 2 multiplexer, which connects LPTIM Input capture 2 to one of the available inputs. For connection details refer to ."]
pub type IC2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - LPTIM input 1 selection The IN1SEL bits control the LPTIM input 1 multiplexer, which connects LPTIM input 1 to one of the available inputs. For connection details refer to ."]
    #[inline(always)]
    pub fn in1sel(&self) -> IN1SEL_R {
        IN1SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - LPTIM input 2 selection The IN2SEL bits control the LPTIM input 2 multiplexer, which connects LPTIM input 2 to one of the available inputs. For connection details refer to ."]
    #[inline(always)]
    pub fn in2sel(&self) -> IN2SEL_R {
        IN2SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 16:17 - LPTIM input capture 1 selection The IC1SEL bits control the LPTIM Input capture 1 multiplexer, which connects LPTIM Input capture 1 to one of the available inputs. For connection details refer to ."]
    #[inline(always)]
    pub fn ic1sel(&self) -> IC1SEL_R {
        IC1SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - LPTIM input capture 2 selection The IC2SEL bits control the LPTIM Input capture 2 multiplexer, which connects LPTIM Input capture 2 to one of the available inputs. For connection details refer to ."]
    #[inline(always)]
    pub fn ic2sel(&self) -> IC2SEL_R {
        IC2SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - LPTIM input 1 selection The IN1SEL bits control the LPTIM input 1 multiplexer, which connects LPTIM input 1 to one of the available inputs. For connection details refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn in1sel(&mut self) -> IN1SEL_W<CFGR2rs> {
        IN1SEL_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - LPTIM input 2 selection The IN2SEL bits control the LPTIM input 2 multiplexer, which connects LPTIM input 2 to one of the available inputs. For connection details refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn in2sel(&mut self) -> IN2SEL_W<CFGR2rs> {
        IN2SEL_W::new(self, 4)
    }
    #[doc = "Bits 16:17 - LPTIM input capture 1 selection The IC1SEL bits control the LPTIM Input capture 1 multiplexer, which connects LPTIM Input capture 1 to one of the available inputs. For connection details refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn ic1sel(&mut self) -> IC1SEL_W<CFGR2rs> {
        IC1SEL_W::new(self, 16)
    }
    #[doc = "Bits 20:21 - LPTIM input capture 2 selection The IC2SEL bits control the LPTIM Input capture 2 multiplexer, which connects LPTIM Input capture 2 to one of the available inputs. For connection details refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn ic2sel(&mut self) -> IC2SEL_W<CFGR2rs> {
        IC2SEL_W::new(self, 20)
    }
}
#[doc = "LPTIM configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR2rs;
impl crate::RegisterSpec for CFGR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr2::R`](R) reader structure"]
impl crate::Readable for CFGR2rs {}
#[doc = "`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure"]
impl crate::Writable for CFGR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR2 to value 0"]
impl crate::Resettable for CFGR2rs {
    const RESET_VALUE: u32 = 0;
}
